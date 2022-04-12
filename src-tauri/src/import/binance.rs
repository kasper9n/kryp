use super::csv::{csv_rows, get_cell, get_cell_index, read_csv_header};
use super::{ImportStatus, ImportTransaction};
use crate::import::ImportData;
use crate::tax::Tax;
use crate::throw;
use crate::transaction::{BaseTransaction, UncostedTransaction, Value};
use chrono::{TimeZone, Utc};
use csv::StringRecord;
use std::fs::File;
use tauri::Window;

pub async fn read(
  mut reader: csv::Reader<File>,
  win: Window,
  tax: &mut Tax,
) -> Result<ImportData, String> {
  let mut rows = csv_rows(&mut reader);
  let cols = CsvCols::from_header(&read_csv_header(&mut rows)?)?;
  let mut uncosted_transactions = Vec::new();

  for (i, row) in rows {
    let uncosted_transaction = match from_row(row?, &cols).await {
      Ok(Some(tx)) => tx,
      Ok(None) => continue,
      Err(e) => throw!("Error in row {}: {}", i + 1, e),
    };
    let import_tx = ImportTransaction::from_uncosted_tx(uncosted_transaction, tax).await;
    uncosted_transactions.push(import_tx);

    win.emit("importStatus", ImportStatus { index: i + 1 }).ok();
  }

  Ok(ImportData::new("Binance", uncosted_transactions))
}

async fn from_row(
  row: StringRecord,
  cols: &CsvCols,
) -> Result<Option<UncostedTransaction>, String> {
  // let user_id = get_cell(&row, Some(cols.user_id), "User_ID")?;
  let utc_time = get_cell(&row, Some(cols.utc_time), "Utc_Time")?;
  let account = get_cell(&row, Some(cols.account), "Account")?;
  let operation = get_cell(&row, Some(cols.operation), "Operation")?;
  let coin = get_cell(&row, Some(cols.coin), "Coin")?;
  let change = get_cell(&row, Some(cols.change), "Change")?;
  let remark = get_cell(&row, Some(cols.remark), "Remark")?;

  let timestamp = match Utc.datetime_from_str(utc_time, "%Y-%m-%d %H:%M:%S") {
    Ok(date) => date.timestamp_millis(),
    Err(e) => throw!("Invalid date: {}", e),
  };

  let mut base_transaction = BaseTransaction {
    tag: "".into(),
    date: timestamp,
    note: remark.into(),
    hash: "".into(),
    sent: None,
    recv: None,
    fee: None,
    manual_worth: None,
  };
  match (account, operation) {
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
    ("Spot", _) => throw!("Unsupported operation: {}", operation),
    _ => throw!("Unsupported Account: {}", account),
  };

  let uncosted_transaction = base_transaction.into_uncosted_transaction()?;
  println!("{:#?}", uncosted_transaction);
  Ok(Some(uncosted_transaction))
}

#[derive(Debug)]
pub struct CsvCols {
  // user_id: usize,
  utc_time: usize,
  account: usize,
  operation: usize,
  coin: usize,
  change: usize,
  remark: usize,
}
impl CsvCols {
  pub fn from_header(header: &StringRecord) -> Result<Self, String> {
    let row: Vec<String> = header.iter().map(|s| s.to_lowercase()).collect();
    println!("{:?}", row);

    Ok(CsvCols {
      // user_id: get_cell_index(&row, &["user_id"])?,
      utc_time: get_cell_index(&row, &["utc_time"])?,
      account: get_cell_index(&row, &["account"])?,
      operation: get_cell_index(&row, &["operation"])?,
      coin: get_cell_index(&row, &["coin"])?,
      change: get_cell_index(&row, &["change"])?,
      remark: get_cell_index(&row, &["remark"])?,
    })
  }
}
