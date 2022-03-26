use crate::data::{to_json, Data};
use crate::fetch_current::fetch_current;
use crate::tax::Tax;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use tauri::{command, State};

#[derive(Serialize, Debug)]
struct Holding {
  asset: String,
  amount: Decimal,
  cost: Decimal,
  value: Option<Decimal>,
  error: Option<String>,
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
}

fn get_holdings_unsorted(tax: &Tax) -> Vec<Holding> {
  let mut holdings_map: HashMap<String, Holding> = HashMap::new();
  for balance in &tax.balances {
    let key = balance.currency.clone();
    if balance.amount > dec!(0) {
      let holding = holdings_map.entry(key.clone()).or_insert(Holding::new(key));
      holding.amount += balance.amount;
      holding.cost += balance.cost;
    }
  }
  holdings_map.into_iter().map(|(_k, v)| v).collect()
}

#[command]
pub async fn get_holdings(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().await;
  let mut holdings = get_holdings_unsorted(&kryp.tax);
  holdings.sort_by(|a, b| a.amount.cmp(&b.amount));
  to_json(&holdings)
}

#[command]
pub async fn get_holdings_valued(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().await;
  let mut holdings = get_holdings_unsorted(&kryp.tax);

  let assets: Vec<_> = holdings.iter().map(|h| &h.asset).collect();
  let prices = fetch_current(assets, &kryp.tax.settings.base_currency).await?;

  for holding in &mut holdings {
    if let Some(price) = prices.get(&holding.asset) {
      holding.value = Some(holding.amount * price);
    } else {
      holding.error = Some("No price".to_string());
    }
  }
  holdings.sort_by(|a, b| b.value.cmp(&a.value));
  to_json(&holdings)
}

#[derive(Serialize, Debug)]
struct WalletHoldings {
  name: String,
  holdings: HashMap<String, Holding>,
}

#[command]
pub async fn get_holdings_by_wallet(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().await;
  let mut wallets_map: HashMap<String, WalletHoldings> = HashMap::new();
  for balance in &kryp.tax.balances {
    let asset = balance.currency.clone();
    if balance.amount > dec!(0) {
      let wallet = wallets_map
        .entry(balance.wallet.clone())
        .or_insert(WalletHoldings {
          name: balance.wallet.clone(),
          holdings: HashMap::new(),
        });
      let holding = wallet
        .holdings
        .entry(asset.clone())
        .or_insert(Holding::new(asset));
      holding.amount += balance.amount;
      holding.cost += balance.cost;
    }
  }
  to_json(&wallets_map)
}
