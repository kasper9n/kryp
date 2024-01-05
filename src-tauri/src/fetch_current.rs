use crate::fetch::{http_get, parse_coin_gecko_error};
use crate::prices::{get_id, symbol_kind, AssetKind};
use crate::{err, throw};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

type CurrentPrices = HashMap<String, Decimal>;

pub async fn fetch_current(assets: Vec<&String>, base: &String) -> Result<CurrentPrices, String> {
	let mut crypto_ids = Vec::new();
	let mut fiat_ids = Vec::new();
	for asset in &assets {
		if let Some(id) = get_id(asset) {
			match symbol_kind(asset) {
				Some(AssetKind::Crypto) => crypto_ids.push(id),
				Some(AssetKind::Fiat) => fiat_ids.push(id),
				None => {}
			}
		}
	}
	let base_id = get_id(base).ok_or("No base ID".to_string())?;
	if !assets.contains(&base) {
		match symbol_kind(&base) {
			Some(AssetKind::Crypto) => crypto_ids.push(base_id),
			Some(AssetKind::Fiat) => fiat_ids.push(base_id),
			None => {}
		}
	}
	let crypto_prices = match coin_gecko(&crypto_ids).await {
		Err(e) => throw!("{}", e),
		Ok(p) => p,
	};
	let fiat_prices = match exchangerate_host().await {
		Err(e) => throw!("{}", e),
		Ok(p) => p,
	};

	let mut id_prices = crypto_prices;
	id_prices.extend(fiat_prices);
	let base_price = id_prices
		.get(base)
		.ok_or("No base price".to_string())?
		.clone();

	let mut prices = HashMap::new();
	for asset in assets {
		if let Some(id) = get_id(asset) {
			if let Some(price) = id_prices.remove(&id) {
				if asset == base {
					prices.insert(asset.clone(), dec!(1));
				} else if asset == "USD" {
					prices.insert(asset.clone(), price);
				} else {
					prices.insert(asset.clone(), price * base_price);
				}
			}
		}
	}
	Ok(prices)
}

async fn exchangerate_host() -> Result<CurrentPrices, Box<dyn Error>> {
	#[derive(Deserialize, Debug)]
	struct Latest {
		success: bool,
		rates: CurrentPrices,
	}

	let request_url = "https://api.exchangerate.host/latest?base=USD";
	let response = http_get(request_url).await?;
	if !response.status().is_success() {
		return err!("Error fetching current prices from Exchanerate.host");
	}
	let latest: Latest = response.json().await?;
	if !latest.success {
		return err!("Unknown error fetching current prices from Exchanerate.host");
	}

	Ok(latest.rates)
}

async fn coin_gecko(ids: &[String]) -> Result<CurrentPrices, Box<dyn Error>> {
	#[derive(Deserialize, Debug)]
	struct PriceObject {
		usd: Decimal,
	}
	type SimplePrices = HashMap<String, PriceObject>;

	let request_url = format!(
		"https://api.coingecko.com/api/v3/simple/price?vs_currencies=USD&ids={ids}",
		ids = ids.join(",")
	);
	let response = http_get(&request_url).await?;
	if !response.status().is_success() {
		let default_err_msg = format!("Error fetching current prices");
		let err_msg = parse_coin_gecko_error(response, &default_err_msg).await;
		return err!("{}", err_msg);
	}
	let simple_prices: SimplePrices = response.json().await?;

	let prices: CurrentPrices = simple_prices
		.into_iter()
		.map(|(id, obj)| (id, obj.usd))
		.collect();
	Ok(prices)
}
