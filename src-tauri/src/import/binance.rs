use crate::import::get_cell_index;
use crate::import::ImportData;
use crate::tax::Tax;
use crate::throw;
use crate::transaction::{BaseTransaction, Value};
use chrono::{TimeZone, Utc};
use csv::StringRecord;
use std::fs::File;
use tauri::Window;

use super::{ImportStatus, ImportTransaction};

pub async fn read(
  mut reader: csv::Reader<File>,
  win: Window,
  tax: &mut Tax,
) -> Result<ImportData, String> {
  let cols = match reader.headers() {
    Ok(header) => CsvCols::from_header(header)?,
    Err(e) => throw!("Unable to read headers: {}", e),
  };
  let header_length = 1;
  let mut has_errors = false;
  let mut uncosted_transactions = Vec::new();

  for (i, record) in reader.records().enumerate() {
    let n = i + 1 + header_length;
    let row = record.map_err(|e| format!("Unable to read row {}: {}", n, e))?;

    let import_transaction = match from_row(row, &cols, &mut *tax).await {
      Ok(Some(tx)) => tx,
      Ok(None) => continue,
      Err(e) => throw!("Error in row {}: {}", n, e),
    };
    if import_transaction.error.is_some() {
      has_errors = true;
    }

    win
      .emit("importStatus", ImportStatus { index: n as u64 })
      .ok();
    uncosted_transactions.push(import_transaction);
  }

  Ok(ImportData {
    transactions: uncosted_transactions.clone(),
    has_errors,
  })
}

async fn from_row(
  row: StringRecord,
  cols: &CsvCols,
  tax: &mut Tax,
) -> Result<Option<ImportTransaction>, String> {
  // let user_id = cols.get(&row, Some(cols.user_id), "User_ID")?;
  let utc_time = cols.get(&row, Some(cols.utc_time), "Utc_Time")?;
  let account = cols.get(&row, Some(cols.account), "Account")?;
  let operation = cols.get(&row, Some(cols.operation), "Operation")?;
  let coin = cols.get(&row, Some(cols.coin), "Coin")?;
  let change = cols.get(&row, Some(cols.change), "Change")?;
  let remark = cols.get(&row, Some(cols.remark), "Remark")?;

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

  let cost = uncosted_transaction.get_or_calculate_cost(
    &mut tax.price_data,
    &tax.settings.apis,
    &tax.settings.base_currency,
  );
  let import_transaction = match cost.await {
    Ok(cost) => ImportTransaction {
      transaction: uncosted_transaction,
      cost: Some(cost),
      error: None,
    },
    Err(e) => ImportTransaction {
      transaction: uncosted_transaction,
      cost: None,
      error: Some(e),
    },
  };
  Ok(Some(import_transaction))
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
  fn get<'a>(
    &self,
    row: &'a StringRecord,
    col: Option<usize>,
    name: &str,
  ) -> Result<&'a str, String> {
    let i = col.ok_or(format!("Missing \"{}\" column", name))?;
    let cell = row.get(i).ok_or(format!("Missing \"{}\" cell", name))?;
    Ok(cell)
  }
}
