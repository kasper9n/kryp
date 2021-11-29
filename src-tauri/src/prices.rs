use chrono::{Duration, NaiveDate, NaiveDateTime};
use reqwest;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
#[cfg(test)]
use std::ops::RangeInclusive;
use std::{thread, time};

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceData {
  assets: HashMap<String, PriceDataAsset>,
}

impl PriceData {
  pub fn new() -> Self {
    return PriceData {
      assets: HashMap::new(),
    };
  }
  pub fn symbol_kind(&mut self, symbol: &str) -> AssetKind {
    return match symbol {
      "USD" | "EUR" | "NOK" => AssetKind::Fiat,
      _ => AssetKind::Crypto,
    };
  }
  pub fn asset(&mut self, symbol: &str) -> &mut PriceDataAsset {
    let symbol = symbol.to_uppercase();
    let kind = self.symbol_kind(&symbol);
    let entry = self.assets.entry(symbol.clone());
    let interval = match kind {
      AssetKind::Fiat => Interval::Daily,
      AssetKind::Crypto => Interval::HourlyOrDaily,
    };
    let price_data_asset = entry.or_insert(PriceDataAsset {
      symbol: symbol.clone(),
      kind,
      interval,
      prices: BTreeMap::new(),
    });
    return price_data_asset;
  }
  pub async fn get_value(
    &mut self,
    amount: Decimal,
    asset: &str,
    date: i64,
    base: &str,
  ) -> Decimal {
    if base == asset {
      amount
    } else if base == "USD" {
      let usd_price = self.get_usd_price_dec(asset, date).await;
      amount * usd_price
    } else {
      let usd_price = self.get_usd_price_dec(asset, date).await;
      let base_usd_price = self.get_usd_price_dec(base, date).await;
      let price = usd_price / base_usd_price;
      amount * price
    }
  }
  pub async fn get_usd_price_dec(&mut self, currency: &str, date: i64) -> Decimal {
    let price = self.get_usd_price(currency, date).await;
    return Decimal::from_f64(price).expect("Error getting price");
  }
  pub async fn get_usd_price(&mut self, currency: &str, date: i64) -> f64 {
    let currency = currency.to_uppercase();
    if currency == "USD" {
      return 1.0;
    }
    let price_data_asset = self.asset(&currency);
    match price_data_asset.local_price(date, false) {
      Some(price) => return price.1,
      None => {
        price_data_asset.fetch(date).await;
        match price_data_asset.local_price(date, true) {
          Some(price) => return price.1,
          None => panic!("No price found"),
        };
      }
    }
  }
}

#[cfg(test)]
macro_rules! map(
  ($($k:expr => $v:expr),* $(,)?) => {
    std::iter::Iterator::collect(std::array::IntoIter::new([$(($k, $v),)*]))
  };
);

#[tokio::test]
async fn get_value() {
  use rust_decimal_macros::dec;
  let mut pd = PriceData::new();
  pd.asset("USD").prices = map! {
    // this should never be used, usd price in usd is always 1
    1640000000000 => 999.0
  };
  pd.asset("NOK").prices = map! {
    1640000000000 => 0.1
  };
  pd.asset("ETH").prices = map! {
    1640000000000 => 5000.0
  };

  assert_eq!(
    pd.get_value(dec!(2), "USD", 1640000000000, "USD").await,
    dec!(2),
    "2 USD in base USD"
  );

  assert_eq!(
    pd.get_value(dec!(2), "NOK", 1640000000000, "USD").await,
    dec!(0.2),
    "2 NOK in base USD"
  );

  assert_eq!(
    pd.get_value(dec!(2), "ETH", 1640000000000, "NOK").await,
    dec!(100_000), // 2 * 5000 / 0.1
    "2 ETH in base NOK"
  );
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceDataAsset {
  pub symbol: String,
  pub kind: AssetKind,
  pub interval: Interval,
  pub prices: BTreeMap<i64, f64>,
}

impl PriceDataAsset {
  /// Returns (date, price)
  pub fn local_price(&self, target_date: i64, extra_tolerance: bool) -> Option<(i64, f64)> {
    let max_offset = match self.interval {
      Interval::Daily => 1000 * 60 * 60 * if extra_tolerance { 25 } else { 12 },
      Interval::HourlyOrDaily => 1000 * 60 * if extra_tolerance { 60 * 25 } else { 50 },
    };
    let number_range = target_date - max_offset..=target_date + max_offset;
    let range = self.prices.range(number_range);
    let closest = range.min_by_key(|x| (target_date - x.0).abs())?;
    return Some((*closest.0, *closest.1));
  }

  async fn fetch(&mut self, date: i64) {
    let result = match self.kind {
      AssetKind::Fiat => self.fetch_fiat(date).await,
      AssetKind::Crypto => self.fetch_crypto(date).await,
    };
    if let Err(e) = result {
      panic!("Error fetching prices: {}", e);
    }
  }

  async fn fetch_fiat(&mut self, date: i64) -> Result<(), reqwest::Error> {
    let start_timestamp = date / 1000 - 60 * 60 * 24 * 10; // 10 days before
    let start_dt = NaiveDateTime::from_timestamp(start_timestamp, 0);
    let start_dt_str = start_dt.format("%Y-%m-%d").to_string();
    println!("{} {}", self.symbol, start_dt_str);
    let end_dt = start_dt + Duration::days(365);

    type PriceMap = HashMap<String, String>;
    #[derive(Deserialize, Debug)]
    struct Timeseries {
      success: bool,
      timeseries: bool,
      rates: HashMap<String, PriceMap>,
    }
    thread::sleep(time::Duration::from_millis(500));
    let request_url = format!(
      "https://api.exchangerate.host/timeseries?base={symbol}&symbols={base}&places=8&start_date={from}&end_date={to}",
      symbol = self.symbol,
      base = "USD",
      from = start_dt_str,
      to = end_dt.format("%Y-%m-%d").to_string(),
    );
    let timeseries_res = reqwest::get(request_url).await?;
    if !timeseries_res.status().is_success() {
      panic!("Error fetching coins {}", self.symbol);
    }
    let timeseries: Timeseries = timeseries_res.json().await?;
    if !timeseries.success || !timeseries.timeseries {
      panic!("Unknown error fetching prices");
    }
    for (date, price_map) in timeseries.rates {
      let rate: f64 = match price_map.get("USD") {
        None => continue,
        Some(rate) => rate.parse().expect("Error parsing price"),
      };
      let d = NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect("Error parsing price time");
      let timestamp = d.and_hms(0, 0, 0).timestamp_millis();
      self.prices.entry(timestamp).or_insert(rate);
    }
    Ok(())
  }

  async fn fetch_crypto(&mut self, date: i64) -> Result<(), reqwest::Error> {
    let start_timestamp = date / 1000 - 60 * 60 * 24 * 1; // 1 day before
    let start_dt = NaiveDateTime::from_timestamp(start_timestamp, 0);
    let start_dt_str = start_dt.format("%Y-%m-%d").to_string();
    println!("{} {}", self.symbol, start_dt_str);
    let end_dt = start_dt + Duration::days(30);

    #[derive(Deserialize, Debug)]
    struct Coin {
      id: String,
      symbol: String,
      name: String,
    }
    let coingecko_duration = time::Duration::from_millis(600);
    thread::sleep(coingecko_duration);
    let request_url = "https://api.coingecko.com/api/v3/coins/list";
    let coins_res = reqwest::get(request_url).await?;
    if !coins_res.status().is_success() {
      if coins_res.status() == 429 {
        panic!("Rate limit, please try again");
      } else {
        panic!("Error fetching coins {}", self.symbol);
      }
    }
    let coins: Vec<Coin> = coins_res.json().await?;

    let mut coin_id_map: HashMap<String, String> = HashMap::new();
    for coin in coins {
      if coin_id_map.contains_key(&coin.symbol) {
        // TODO Handle duplicate tickers (instead of ignoring them)
        coin_id_map.insert(coin.symbol.clone(), "".to_string());
      } else {
        coin_id_map.insert(coin.symbol.to_uppercase(), coin.id);
      }
    }
    let id = coin_id_map
      .get(&self.symbol)
      .expect(&format!("No coin ID found for {}", self.symbol));
    if id == "" {
      panic!("Ticker {} has multiple coins", self.symbol);
    }

    #[derive(Deserialize, Debug)]
    struct MarketChart {
      prices: Vec<(i64, f64)>,
    }
    thread::sleep(coingecko_duration);
    let request_url = format!(
      "https://api.coingecko.com/api/v3/coins/{id}/market_chart/range?vs_currency={base}&from={from}&to={to}",
      id = id,
      base = "USD",
      from = start_dt.timestamp(),
      to = end_dt.timestamp(),
    );
    let market_chart_res = reqwest::get(request_url).await?;
    if !market_chart_res.status().is_success() {
      if market_chart_res.status() == 429 {
        panic!("Rate limit, please try again");
      } else {
        panic!("Error fetching {} prices", self.symbol);
      }
    }
    let market_chart: MarketChart = market_chart_res.json().await?;

    for (price_date, price_rate) in market_chart.prices {
      self.prices.entry(price_date).or_insert(price_rate);
    }

    return Ok(());
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Interval {
  Daily = 0,
  HourlyOrDaily = 1,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum AssetKind {
  Fiat = 0,
  Crypto = 1,
}

#[test]
fn crypto() {
  let mut pd = PriceData::new();
  let mut eth_pda = pd.asset("ETH");
  eth_pda.prices = map! {
    1600000000000 => 5.2,
    1600009000000 => 6.1
  };
  assert_eq!(eth_pda.local_price(1600000000000, false).unwrap().1, 5.2);
  assert_eq!(eth_pda.local_price(1600008000000, false).unwrap().1, 6.1);
  assert_eq!(eth_pda.local_price(1610000000000, false), None);
}

/// Get a range from -x% to +x% of value. For example, a 50% tolerance
/// around 10 gives you 5..15 (not 5..20, which might be better for prices)
#[cfg(test)]
fn tolerance_pct(num: f64, tolerance_percent: f64) -> RangeInclusive<f64> {
  let min = num - num * tolerance_percent / 100.0;
  let max = num + num * tolerance_percent / 100.0;
  return min..=max;
}

#[tokio::test]
async fn api_fetch() {
  let mut pd = PriceData::new();
  let date = chrono::NaiveDate::from_ymd(2020, 01, 10).and_hms(0, 0, 0);
  let nok_price = pd.get_usd_price("NOK", date.timestamp_millis()).await;
  assert!(tolerance_pct(0.1125, 0.2).contains(&nok_price));
  let eth_price = pd.get_usd_price("ETH", date.timestamp_millis()).await;
  assert!(tolerance_pct(137.5, 1.0).contains(&eth_price));
}
