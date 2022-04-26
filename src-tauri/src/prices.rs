use crate::tax::Api;
use crate::{fetch, throw};
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceData {
  assets: HashMap<String, PriceDataAsset>,
}

lazy_static! {
  static ref FIAT_LIST: FiatList = get_fiat_list();
  static ref CRYPTO_LIST: CryptoList = get_crypto_list();
}

type FiatList = HashMap<String, String>;
fn get_fiat_list() -> FiatList {
  let fiat_list_json: &str = include_str!("../../public/assets/fiat-list.json");
  serde_json::from_str(&fiat_list_json).unwrap()
}

#[derive(Deserialize)]
struct CryptoInfo {
  symbol: String,
  name: String,
  coingecko_id: String,
}
type CryptoList = HashMap<String, CryptoInfo>;

fn get_crypto_list() -> CryptoList {
  let crypto_list_csv: &str = include_str!("../../public/assets/crypto-list.csv");
  let mut reader = csv::ReaderBuilder::new()
    .delimiter(',' as u8)
    .has_headers(true)
    .from_reader(crypto_list_csv.as_bytes());

  let mut map = HashMap::new();
  for row in reader.deserialize() {
    let item: CryptoInfo = row.expect("Error reading csv");
    let inserted = map.insert(item.symbol.clone(), item);
    assert!(inserted.is_none(), "Duplicate symbol");
  }
  map
}

pub fn symbol_kind(symbol: &str) -> Option<AssetKind> {
  if FIAT_LIST.contains_key(symbol) {
    Some(AssetKind::Fiat)
  } else if CRYPTO_LIST.contains_key(symbol) {
    Some(AssetKind::Crypto)
  } else {
    None
  }
}
pub fn get_id(symbol: &str) -> Option<String> {
  if let Some(_) = FIAT_LIST.get(symbol) {
    Some(symbol.to_string())
  } else if let Some(crypto_asset) = CRYPTO_LIST.get(symbol) {
    Some(crypto_asset.coingecko_id.to_string())
  } else {
    None
  }
}
fn get_name(symbol: &str) -> Option<String> {
  if let Some(fiat_currency) = FIAT_LIST.get(symbol) {
    Some(fiat_currency.clone())
  } else if let Some(crypto_asset) = CRYPTO_LIST.get(symbol) {
    Some(crypto_asset.name.to_string())
  } else {
    None
  }
}

impl PriceData {
  pub fn new() -> Self {
    return PriceData {
      assets: HashMap::new(),
    };
  }
  pub fn list_assets(&self) -> Vec<&String> {
    self.assets.keys().collect()
  }
  pub fn get_asset(&self, symbol: &str) -> Option<&PriceDataAsset> {
    let symbol = symbol.to_uppercase();
    self.assets.get(&symbol)
  }
  pub fn asset(&mut self, symbol: &str) -> Result<&mut PriceDataAsset, String> {
    let symbol = symbol.to_uppercase();
    let kind = symbol_kind(&symbol).ok_or(format!("Unsupported asset \"{}\"", symbol))?;
    let entry = self.assets.entry(symbol.clone());
    let interval = match kind {
      AssetKind::Fiat => Interval::Daily,
      AssetKind::Crypto => Interval::HourlyOrDaily,
    };
    let price_data_asset = entry.or_insert(PriceDataAsset {
      symbol: symbol.clone(),
      name: get_name(&symbol).ok_or(format!("Unsupported asset \"{}\"", symbol))?,
      id: get_id(&symbol).ok_or(format!("Unsupported asset \"{}\"", symbol))?,
      kind,
      interval,
      prices: BTreeMap::new(),
    });
    Ok(price_data_asset)
  }
  pub async fn get_value(
    &mut self,
    amount: Decimal,
    asset: &str,
    date: i64,
    apis: &[Api],
    base: &str,
  ) -> Result<Decimal, String> {
    if base == asset {
      Ok(amount)
    } else if base == "USD" {
      let usd_price = self.get_usd_price_dec(asset, date, apis).await?;
      Ok(amount * usd_price)
    } else {
      let usd_price = self.get_usd_price_dec(asset, date, apis).await?;
      let base_usd_price = self.get_usd_price_dec(base, date, apis).await?;
      let price = usd_price / base_usd_price;
      Ok(amount * price)
    }
  }
  async fn get_usd_price_dec(
    &mut self,
    currency: &str,
    date: i64,
    apis: &[Api],
  ) -> Result<Decimal, String> {
    let price = self.get_usd_price(currency, date, apis).await?;
    match Decimal::from_f64(price) {
      Some(price) => Ok(price),
      None => throw!("Unable to convert price from float to decimal"),
    }
  }
  async fn get_usd_price(
    &mut self,
    currency: &str,
    date: i64,
    apis: &[Api],
  ) -> Result<f64, String> {
    let currency = currency.to_uppercase();
    if currency == "USD" {
      return Ok(1.0);
    }
    let price_data_asset = self.asset(&currency)?;

    let mut errors = Vec::new();

    for api in apis {
      if api.asset_kind() == price_data_asset.kind {
        match price_data_asset.local_price(date, false) {
          Some(price) => return Ok(price.1),
          None => {
            match price_data_asset.fetch(date, api).await {
              Ok(()) => {}
              Err(e) => {
                let msg = format!("Error fetching price of {}: {}", currency, e);
                eprintln!("{}", msg);
                errors.push(msg);
              }
            };
            match price_data_asset.local_price(date, true) {
              Some(price) => return Ok(price.1),
              None => continue,
            };
          }
        }
      }
    }
    if errors.len() == 0 {
      let naive_dt = NaiveDateTime::from_timestamp(date / 1000, 0);
      let date_str = naive_dt.format("%Y-%m-%d").to_string();
      throw!("No price found for {} at {}", currency, date_str);
    } else if errors.len() == 1 {
      throw!("{}", errors[0]);
    } else {
      throw!("{}", errors.join("\n"));
    }
  }
}

#[cfg(test)]
macro_rules! map(
  ($($k:expr => $v:expr),* $(,)?) => {
    std::iter::Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
  };
);

#[tokio::test]
async fn get_value() {
  use rust_decimal_macros::dec;
  let mut pd = PriceData::new();
  pd.asset("USD").unwrap().prices = map! {
    // this should never be used, usd price in usd is always 1
    1640000000000 => 999.0
  };
  pd.asset("NOK").unwrap().prices = map! {
    1640000000000 => 0.1
  };
  pd.asset("ETH").unwrap().prices = map! {
    1640000000000 => 5000.0
  };

  let apis = &crate::tax::Tax::new("USD").settings.apis;

  assert_eq!(
    pd.get_value(dec!(2), "USD", 1640000000000, apis, "USD")
      .await
      .unwrap(),
    dec!(2),
    "2 USD in base USD"
  );

  assert_eq!(
    pd.get_value(dec!(2), "NOK", 1640000000000, apis, "USD")
      .await
      .unwrap(),
    dec!(0.2),
    "2 NOK in base USD"
  );

  assert_eq!(
    pd.get_value(dec!(2), "ETH", 1640000000000, apis, "NOK")
      .await
      .unwrap(),
    dec!(100_000), // 2 * 5000 / 0.1
    "2 ETH in base NOK"
  );
}

pub type Prices = BTreeMap<i64, f64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceDataAsset {
  pub name: String,
  pub symbol: String,
  pub id: String,
  pub kind: AssetKind,
  pub interval: Interval,
  pub prices: Prices,
}

impl PriceDataAsset {
  /// Returns (date, price)
  pub fn local_price(&self, target_date: i64, extra_tolerance: bool) -> Option<(i64, f64)> {
    let minute = 1000 * 60;
    let hour = minute * 60;
    // TODO separate price data for different intervals
    let max_offset = if extra_tolerance {
      hour * 25
    } else {
      hour * 12
    };
    let number_range = target_date - max_offset..=target_date + max_offset;
    let range = self.prices.range(number_range);
    let closest = range.min_by_key(|x| (target_date - x.0).abs())?;
    return Some((*closest.0, *closest.1));
  }

  async fn fetch(&mut self, date: i64, api: &Api) -> Result<(), String> {
    match fetch::fetch_prices(&self, api, date).await {
      Ok(prices) => {
        for (timestamp, rate) in prices {
          self.prices.entry(timestamp).or_insert(rate);
        }
        Ok(())
      }
      Err(e) => return Err(e),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Interval {
  Daily = 0,
  HourlyOrDaily = 1,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum AssetKind {
  Fiat = 0,
  Crypto = 1,
}

#[test]
fn crypto() {
  let mut pd = PriceData::new();
  let mut eth_pda = pd.asset("ETH").unwrap();
  eth_pda.prices = map! {
    1600000000000 => 5.2,
    1600009000000 => 6.1
  };
  assert_eq!(eth_pda.local_price(1600000000000, false).unwrap().1, 5.2);
  assert_eq!(eth_pda.local_price(1600008000000, false).unwrap().1, 6.1);
  assert_eq!(eth_pda.local_price(1610000000000, false), None);
}

#[cfg(test)]
mod api_fetch {
  use crate::prices::PriceData;
  use crate::tax::{Api, ApiName};
  use std::ops::RangeInclusive;

  /// Get a range from -x% to +x% of value. For example, a 50% tolerance
  /// around 10 gives you 5..15 (not 5..20, which might be better for prices)
  #[cfg(test)]
  fn tolerance_pct(num: f64, tolerance_percent: f64) -> RangeInclusive<f64> {
    let min = num - num * tolerance_percent / 100.0;
    let max = num + num * tolerance_percent / 100.0;
    return min..=max;
  }

  #[tokio::test]
  async fn exchangerate_host() {
    let mut pd = PriceData::new();
    let date = chrono::NaiveDate::from_ymd(2020, 01, 10).and_hms(0, 0, 0);

    let exchangerate_host = Api::new(ApiName::ExchangerateHost);
    let nok_price = pd
      .get_usd_price("NOK", date.timestamp_millis(), &[exchangerate_host])
      .await
      .unwrap();
    assert!(tolerance_pct(0.1125, 0.2).contains(&nok_price));
  }

  #[tokio::test]
  async fn coin_gecko() {
    let mut pd = PriceData::new();
    let date = chrono::NaiveDate::from_ymd(2020, 01, 10).and_hms(0, 0, 0);
    let coin_gecko = Api::new(ApiName::CoinGecko);
    let eth_price = pd
      .get_usd_price("ETH", date.timestamp_millis(), &[coin_gecko])
      .await
      .unwrap();
    assert!(tolerance_pct(137.5, 1.0).contains(&eth_price));
  }

  #[tokio::test]
  async fn crypto_compare() {
    let mut pd = PriceData::new();
    let date = chrono::NaiveDate::from_ymd(2020, 01, 10).and_hms(0, 0, 0);
    let crypto_compare = Api::new(ApiName::CryptoCompare);
    let eth_price = pd
      .get_usd_price("ETH", date.timestamp_millis(), &[crypto_compare])
      .await
      .unwrap();
    assert!(tolerance_pct(137.5, 1.0).contains(&eth_price));
  }
}
