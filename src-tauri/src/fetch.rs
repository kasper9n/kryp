use crate::prices::{AssetKind, PriceDataAsset, Prices};
use crate::tax::{Api, ApiName};
use crate::{err, throw};
use chrono::{Duration, NaiveDate, NaiveDateTime};
use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use std::{thread, time};

pub async fn fetch_prices(pda: &PriceDataAsset, api: &Api, date: i64) -> Result<Prices, String> {
	let naive_dt = NaiveDateTime::from_timestamp_opt(date / 1000, 0).unwrap();
	let date_str = naive_dt.format("%Y-%m-%d").to_string();
	println!("Fetch {:?} prices {} {}", api.name, pda.symbol, date_str);

	let result = match pda.kind {
		AssetKind::Fiat => match api.name {
			ApiName::ExchangerateHost => ecb(pda, date).await,
			_ => throw!("Asset type not supported for this API"),
		},
		AssetKind::Crypto => match api.name {
			ApiName::CoinGecko => coin_gecko(pda, date).await,
			ApiName::CryptoCompare => crypto_compare(pda, date).await,
			_ => throw!("Asset type not supported for this API"),
		},
	};

	match result {
		Err(e) => Err(format!("{}", e)),
		Ok(prices) => Ok(prices),
	}
}

pub async fn http_get(url: &str) -> Result<reqwest::Response, reqwest::Error> {
	println!("Fetch {}", url);
	reqwest::get(url).await
}

async fn ecb(pda: &PriceDataAsset, date: i64) -> Result<Prices, Box<dyn Error>> {
	thread::sleep(time::Duration::from_millis(500));

	let start_timestamp = date / 1000 - 60 * 60 * 24 * 10; // 10 days before
	let start_dt = NaiveDateTime::from_timestamp_opt(start_timestamp, 0).unwrap();
	let start_dt_str = start_dt.format("%Y-%m-%d").to_string();
	let end_dt = start_dt + Duration::days(365);

	// https://data.ecb.europa.eu/data/datasets/EXR/structure
	// https://data.ecb.europa.eu/help/api/overview
	let request_url = format!(
		"https://data-api.ecb.europa.eu/service/data/EXR/D.{symbol}.{base}.SP00.A?startPeriod={from}&endPeriod={to}&format=jsondata",
		symbol = pda.id,
		from = start_dt_str,
		base = "EUR", // Only EUR is supported
		to = end_dt.format("%Y-%m-%d").to_string(),
	);
	let timeseries_res = http_get(&request_url).await?;
	if !timeseries_res.status().is_success() {
		return err!("Error fetching price of {}", pda.id);
	}
	let timeseries_body = timeseries_res.bytes().await?;

	// TODO convert EUR to be based in USD

	let mut timeseries_reader = csv::Reader::from_reader(timeseries_body.as_ref());
	let mut prices = Prices::new();
	for row in timeseries_reader.deserialize::<EcbExchangeRateRow>() {
		let row = row.or_else(|e| throw!("Error parsing ECV data: {}", e))?;
		if row.freq != "D" {
			return err!("Unexpected frequency: {}", row.freq);
		}

		let rate: f64 = row.obs_value.parse().expect("Error parsing price");
		let d = NaiveDate::parse_from_str(&row.time_period, "%Y-%m-%d")?;
		// ECB data is the spot price average (SP00.A), so we use the middle of the day
		let timestamp = d.and_hms_opt(12, 0, 0).unwrap().timestamp_millis();
		prices.entry(timestamp).or_insert(rate);
	}
	Ok(prices)
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct EcbExchangeRateRow {
	key: String,
	freq: String,
	currency: String,
	currency_denom: String,
	exr_type: String,
	exr_suffix: String,
	time_period: String,
	obs_value: String,
	obs_status: String,
	obs_conf: String,
	obs_pre_break: String,
	obs_com: String,
	time_format: String,
	breaks: String,
	collection: String,
	compiling_org: String,
	diss_org: String,
	dom_ser_ids: String,
	publ_ecb: String,
	publ_mu: String,
	publ_public: String,
	unit_index_base: String,
	compilation: String,
	coverage: String,
	decimals: String,
	nat_title: String,
	source_agency: String,
	source_pub: String,
	title: String,
	title_compl: String,
	unit: String,
	unit_mult: String,
}

#[derive(Deserialize, Debug)]
struct MarketChart {
	prices: Vec<(i64, f64)>,
}

pub async fn parse_coin_gecko_error(response: reqwest::Response, fallback: &str) -> String {
	if response.status() == 429 {
		return "Rate limit, please try again".to_string();
	}
	let json: Value = match response.json().await {
		Ok(value) => value,
		Err(_) => return fallback.to_string(),
	};
	match json {
		Value::Object(obj) => match obj.get("error") {
			Some(error) => match error {
				Value::String(error) => return error.to_string(),
				_ => {}
			},
			None => {}
		},
		_ => {}
	}
	fallback.to_string()
}

async fn coin_gecko(pda: &PriceDataAsset, date: i64) -> Result<Prices, Box<dyn Error>> {
	thread::sleep(time::Duration::from_millis(600));

	let start_timestamp = date / 1000 - 60 * 60 * 24 * 3; // 3 day before
	let start_dt = NaiveDateTime::from_timestamp_opt(start_timestamp, 0).unwrap();
	let end_dt = start_dt + Duration::days(30);

	let request_url = format!(
		"https://api.coingecko.com/api/v3/coins/{id}/market_chart/range?vs_currency={base}&from={from}&to={to}",
		id = pda.id,
		base = "USD",
		from = start_dt.timestamp(),
		to = end_dt.timestamp(),
	);

	let market_chart_res = http_get(&request_url).await?;
	if !market_chart_res.status().is_success() {
		let default_err_msg = format!("Error fetching {} prices", pda.id);
		let err_msg = parse_coin_gecko_error(market_chart_res, &default_err_msg).await;
		return err!("{}", err_msg);
	}
	let market_chart: MarketChart = market_chart_res.json().await?;

	let mut prices = Prices::new();
	for (price_date, price_rate) in market_chart.prices {
		prices.entry(price_date).or_insert(price_rate);
	}

	Ok(prices)
}

async fn crypto_compare(pda: &PriceDataAsset, date: i64) -> Result<Prices, Box<dyn Error>> {
	thread::sleep(time::Duration::from_millis(100));

	let to_timestamp = date / 1000 + 60 * 60 * 24 * 15; // 15 day after
	let limit = 24 * 30; // 30 days of results

	let request_url = format!(
		"https://min-api.cryptocompare.com/data/v2/histohour?fsym={id}&tsym={base}&limit={limit}&toTs={to}",
		id = pda.symbol,
		base = "USD",
		limit = limit,
		to = to_timestamp,
	);

	#[derive(Deserialize, Debug)]
	struct Ohlcv {
		time: i64,
		close: f64,
	}

	#[derive(Deserialize, Debug)]
	struct Data {
		#[serde(alias = "Data")]
		data: Option<Vec<Ohlcv>>,
	}

	#[derive(Deserialize, Debug)]
	struct Res {
		#[serde(alias = "Type")]
		kind: i16,
		#[serde(alias = "Message")]
		message: String,
		#[serde(alias = "Data")]
		data: Option<Data>,
	}

	let res: Res = http_get(&request_url).await?.json().await?;
	if res.kind != 100 {
		return err!("{}", res.message);
	}
	let error_msg = format!("No data: {}", res.message);
	let data = res.data.ok_or(error_msg.clone())?.data.ok_or(error_msg)?;

	let mut prices = Prices::new();
	for ohlcv in data {
		prices.entry(ohlcv.time * 1000).or_insert(ohlcv.close);
	}

	return Ok(prices);
}
