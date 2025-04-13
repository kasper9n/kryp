use crate::calc::Balance;
use crate::data::{to_json, Data};
use crate::fetch_current::fetch_current;
use rust_decimal::{Decimal, RoundingStrategy::AwayFromZero as AwayFrom0};
use rust_decimal_macros::dec;
use serde;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use tauri::{command, State};

#[derive(Serialize, Debug, specta::Type)]
pub struct Holding {
	pub asset: String,
	pub amount: Decimal,
	pub cost: Decimal,
	pub value: Option<Decimal>,
	pub error: Option<String>,
}
impl Holding {
	fn new(key: String) -> Self {
		Self {
			asset: key,
			amount: dec!(0),
			cost: dec!(0),
			value: None,
			error: None,
		}
	}
	pub fn round(&mut self) {
		self.cost = self.cost.round_dp_with_strategy(2, AwayFrom0);
		self.value
			.map(|value| value.round_dp_with_strategy(2, AwayFrom0));
	}
}

#[derive(Default, Serialize, specta::Type)]
pub struct Holdings {
	pub list: Vec<Holding>,
	pub total_cost: Decimal,
	pub total_value: Option<Decimal>,
}
impl Holdings {
	pub fn sort(&mut self) {
		self.list.sort_by(|a, b| b.amount.cmp(&a.amount));
		self.list.sort_by(|a, b| b.value.cmp(&a.value));
	}
}

pub fn holdings_from_balances<B: Borrow<Balance>>(balances: &Vec<B>) -> Holdings {
	let mut map = HashMap::new();
	for balance in balances {
		let balance = balance.borrow();
		let holding = map
			.entry(balance.currency.clone())
			.or_insert(Holding::new(balance.currency.clone()));
		holding.amount += balance.amount;
		holding.cost += balance.cost;
	}
	let mut holdings = Holdings {
		list: map.into_values().collect(),
		total_cost: dec!(0),
		total_value: None,
	};
	holdings.sort();
	holdings
}

#[command]
#[specta::specta]
pub async fn get_holdings(kryp: State<'_, Data>) -> Result<Holdings, String> {
	let kryp = kryp.0.lock().await;
	let mut holdings = holdings_from_balances(&kryp.tax.balances);
	for holding in &mut holdings.list {
		holding.round();
	}
	Ok(holdings)
}

#[command]
#[specta::specta]
pub async fn get_holdings_valued(kryp: State<'_, Data>) -> Result<Holdings, String> {
	let kryp = kryp.0.lock().await;
	let mut holdings = holdings_from_balances(&kryp.tax.balances);

	let assets: Vec<_> = holdings.list.iter().map(|h| &h.asset).collect();
	let prices = fetch_current(assets, &kryp.tax.settings.base_currency).await?;

	for holding in &mut holdings.list {
		if let Some(price) = prices.get(&holding.asset) {
			holding.value = Some(holding.amount * price);
		} else {
			holding.error = Some("No price".to_string());
		}
	}

	holdings.sort();
	for holding in &mut holdings.list {
		holding.round();
	}
	Ok(holdings)
}

#[derive(Serialize, specta::Type)]
pub struct WalletHoldings {
	name: String,
	holdings: Holdings,
}

#[command]
#[specta::specta]
pub async fn get_holdings_by_wallet(
	kryp: State<'_, Data>,
) -> Result<HashMap<String, WalletHoldings>, String> {
	let kryp = kryp.0.lock().await;

	let balances = &kryp.tax.balances;
	let wallets: HashSet<_> = kryp.tax.balances.iter().map(|b| &b.wallet).collect();

	let mut wallets_map: HashMap<String, WalletHoldings> = HashMap::new();
	for wallet in wallets {
		let balances: Vec<&Balance> = balances.iter().filter(|b| &b.wallet == wallet).collect();
		let holdings = holdings_from_balances(&balances);
		let mut wallet_holdings = WalletHoldings {
			name: wallet.clone(),
			holdings,
		};
		for holding in &mut wallet_holdings.holdings.list {
			holding.round();
		}
		wallets_map.insert(wallet.clone(), wallet_holdings);
	}
	Ok(wallets_map)
}
