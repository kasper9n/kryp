use crate::data::{to_json, Data};
use crate::tax::Tax;
use crate::transaction::Transaction;
use serde::Deserialize;
use serde_json::Value;
use tauri::{command, State};

fn get(tax: &Tax, search: Search) -> Vec<&Transaction> {
  let mut transactions = Vec::new();
  for transaction in &tax.transactions {
    if search.tags.len() > 0 && !search.tags.contains(transaction.tag()) {
      continue;
    }
    if search.asset != ""
      && transaction.recv_asset() != Some(&search.asset)
      && transaction.sent_asset() != Some(&search.asset)
      && transaction.fee_asset() != Some(&search.asset)
    {
      continue;
    }
    transactions.push(transaction);
  }
  transactions
}

#[derive(Deserialize)]
pub struct Search {
  tags: Vec<String>,
  asset: String,
}

#[command]
pub async fn get_transactions(kryp: State<'_, Data>, search: Search) -> Result<Value, String> {
  let kryp = kryp.0.lock().await;
  let transactions = get(&kryp.tax, search);
  to_json(&transactions)
}
