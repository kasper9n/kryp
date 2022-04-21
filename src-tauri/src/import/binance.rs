use super::csv::{lowercase_header_contains, read_csv};
use crate::transaction::{BaseTransaction, Quantity, UncostedTransaction, Value};
use crate::{err, throw};
use chrono::{TimeZone, Utc};
use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub async fn read(path: PathBuf) -> Result<Vec<UncostedTransaction>, Box<dyn Error>> {
  let mut csv = read_csv(path)?;
  if lowercase_header_contains(&mut csv, "price") {
    return parse_trade_history(csv).await;
  } else {
    return parse_all_statements(csv).await;
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllStatementsRow {
  // #[serde(rename = "User_ID")]
  // user_id: String,
  #[serde(rename = "UTC_Time")]
  utc_time: String,
  account: String,
  operation: String,
  coin: String,
  change: String,
  remark: String,
}

async fn parse_all_statements(
  mut csv: Reader<File>,
) -> Result<Vec<UncostedTransaction>, Box<dyn Error>> {
  let mut uncosted_transactions = Vec::new();
  for (i, row) in csv.deserialize().enumerate() {
    let uncosted_transaction = match parse_all_statements_row(row?).await {
      Ok(Some(tx)) => tx,
      Ok(None) => continue,
      Err(e) => return err!("Error in row {}: {}", i + 2, e),
    };
    uncosted_transactions.push(uncosted_transaction);
  }

  Ok(uncosted_transactions)
}

async fn parse_all_statements_row(
  row: AllStatementsRow,
) -> Result<Option<UncostedTransaction>, String> {
  let timestamp = match Utc.datetime_from_str(&row.utc_time, "%Y-%m-%d %H:%M:%S") {
    Ok(date) => date.timestamp_millis(),
    Err(e) => throw!("Invalid date: {}", e),
  };
  let change = row.change;
  let coin = row.coin;

  let mut base_transaction = BaseTransaction {
    tag: "".into(),
    date: timestamp,
    note: row.remark,
    hash: "".into(),
    sent: None,
    recv: None,
    fee: None,
    manual_worth: None,
  };
  match (row.account.as_str(), row.operation.as_str()) {
    ("Spot", "Deposit") => {
      base_transaction.tag = "Deposit".into();
      base_transaction.recv = Some(Value::new(change, coin, "Binance")?);
    }
    ("Spot", "Distribution") => {
      base_transaction.tag = "Gift".into();
      base_transaction.recv = Some(Value::new(change, coin, "Binance")?);
    }
    ("Spot", "Savings Interest" | "POS savings interest" | "Launchpool Interest") => {
      base_transaction.tag = "Interest".into();
      base_transaction.recv = Some(Value::new(change, coin, "Binance")?);
    }
    // skip savings balance "lock-ins"
    ("Spot", "Savings purchase" | "POS savings purchase") => {
      return Ok(None);
    }
    // skip savings balance "lock-in releases"
    ("Spot", "Savings Principal redemption" | "POS savings redemption") => {
      return Ok(None);
    }
    // skip trades - impossible to correctly parse
    ("Spot", "Buy" | "Transaction Related" | "Fee") => {
      return Ok(None);
    }
    ("Spot", "Small assets exchange BNB") => {
      let value = Value::new(change, coin, "Binance")?;
      if value.amount.is_sign_positive() {
        base_transaction.tag = "Deposit".into();
        base_transaction.recv = Some(value);
      } else {
        base_transaction.tag = "Withdrawal".into();
        base_transaction.sent = Some(value);
      }
    }
    ("Spot", "Withdraw") => {
      base_transaction.tag = "Withdrawal".into();
      base_transaction.sent = Some(Value::new(change, coin, "Binance")?);
    }
    ("Spot", _) => throw!("Unsupported operation: {}", row.operation),
    (_, _) => throw!("Unsupported Account: {}", row.account),
  };
  if let Some(sent) = &mut base_transaction.sent {
    sent.amount = -sent.amount;
  }
  if let Some(fee) = &mut base_transaction.fee {
    fee.amount = -fee.amount;
  }

  let uncosted_transaction = base_transaction.into_uncosted_transaction()?;
  println!("{:#?}", uncosted_transaction);
  Ok(Some(uncosted_transaction))
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TradeHistoryRow {
  #[serde(rename = "Date(UTC)")]
  date_utc: String,
  // pair: String,
  side: Side,
  // price: String,
  executed: String,
  amount: String,
  fee: String,
}
#[derive(Deserialize)]
enum Side {
  BUY,
  SELL,
}

async fn parse_trade_history(
  mut csv: Reader<File>,
) -> Result<Vec<UncostedTransaction>, Box<dyn Error>> {
  let mut uncosted_transactions = Vec::new();

  for (i, row) in csv.deserialize().enumerate() {
    let uncosted_transaction = match parse_trade_history_row(row?).await {
      Ok(tx) => tx,
      Err(e) => return err!("Error in row {}: {}", i + 2, e),
    };
    uncosted_transactions.push(uncosted_transaction);
  }

  Ok(uncosted_transactions)
}

async fn parse_trade_history_row(row: TradeHistoryRow) -> Result<UncostedTransaction, String> {
  let timestamp = match Utc.datetime_from_str(&row.date_utc, "%Y-%m-%d %H:%M:%S") {
    Ok(date) => date.timestamp_millis(),
    Err(e) => throw!("Invalid date: {}", e),
  };

  let executed = Quantity::parse(&row.executed)?;
  let amount = Quantity::parse(&row.amount)?;

  let (from, to) = match row.side {
    Side::BUY => (amount, executed),
    Side::SELL => (executed, amount),
  };

  let base_transaction = BaseTransaction {
    tag: "Trade".into(),
    date: timestamp,
    note: "".into(),
    hash: "".into(),
    sent: Some(from.with_wallet("Binance")),
    recv: Some(to.with_wallet("Binance")),
    fee: Quantity::parse_optional(&row.fee)?,
    manual_worth: None,
  };

  let uncosted_transaction = base_transaction.into_uncosted_transaction()?;
  println!("{:#?}", uncosted_transaction);
  Ok(uncosted_transaction)
}
