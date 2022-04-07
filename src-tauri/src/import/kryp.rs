use crate::import::{ImportData, ImportStatus, ImportTransaction};
use crate::tax::Tax;
use crate::throw;
use crate::transaction::{BaseTransaction, Quantity, UncostedTransaction, Value};
use chrono::{Local, NaiveDateTime, TimeZone};
use csv::StringRecord;
use std::fs::File;
use tauri::Window;

pub async fn read(
  mut reader: csv::Reader<File>,
  win: Window,
  tax: &mut Tax,
) -> Result<ImportData, String> {
  let header = match reader.headers() {
    Ok(h) => h,
    Err(e) => throw!("Error reading file: {}", e),
  };
  let cols = CsvCols::from_header(header)?;
  println!("{:?}", cols);

  let mut has_errors = false;
  let header_length = 1;
  let mut i = 1;
  let mut uncosted_transactions = Vec::new();
  for record in reader.records() {
    let row = record.map_err(|e| format!("Unable to read row {}: {}", i + header_length, e))?;

    let uncosted_transaction = from_csv_record(row, &cols)
      .await
      .map_err(|e| format!("Error in row {}: {}", i + header_length, e))?;

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
    if import_transaction.error.is_some() {
      has_errors = true;
    }
    win.emit("importStatus", ImportStatus { index: i }).ok();
    uncosted_transactions.push(import_transaction);
    i += 1;
  }
  Ok(ImportData {
    transactions: uncosted_transactions.clone(),
    has_errors,
  })
}

fn pos(row: &Vec<String>, values: &[&str]) -> Option<usize> {
  row.iter().position(|s| values.contains(&s.as_str()))
}

#[derive(Debug)]
pub struct CsvCols {
  kind: Option<usize>,
  date: Option<usize>,
  note: Option<usize>,
  hash: Option<usize>,
  sent_amount: Option<usize>,
  sent_asset: Option<usize>,
  sent_wallet: Option<usize>,
  recv_amount: Option<usize>,
  recv_asset: Option<usize>,
  recv_wallet: Option<usize>,
  fee_amount: Option<usize>,
  fee_asset: Option<usize>,
  cost: Option<usize>,
}
impl CsvCols {
  pub fn from_header(header: &StringRecord) -> Result<Self, String> {
    let row: Vec<String> = header.iter().map(|s| s.to_lowercase()).collect();
    println!("{:?}", row);

    let kind = row.iter().position(|s| s == "type");
    let date = row.iter().position(|s| s == "date");
    let note = row.iter().position(|s| s == "note");
    let hash = row.iter().position(|s| s == "hash" || s == "tx hash");

    let sent_amount = row.iter().position(|s| s == "sent");
    let mut sent_asset = pos(&row, &["sent asset", "r asset"]);
    let mut sent_wallet = pos(&row, &["sent wallet", "r wallet"]);
    if let Some(i) = sent_amount {
      if sent_asset.is_none() && row.get(i + 1) == Some(&"asset".to_string()) {
        sent_asset = Some(i + 1);
      }
      if sent_wallet.is_none() && row.get(i + 2) == Some(&"wallet".to_string()) {
        sent_wallet = Some(i + 2);
      }
    }

    let recv_amount = row.iter().position(|s| s == "received");
    let mut recv_asset = pos(&row, &["received asset", "recv asset", "r asset"]);
    let mut recv_wallet = pos(&row, &["received wallet", "recv wallet", "r wallet"]);
    if let Some(i) = recv_amount {
      if recv_asset.is_none() && row.get(i + 1) == Some(&"asset".to_string()) {
        recv_asset = Some(i + 1);
      }
      if recv_wallet.is_none() && row.get(i + 2) == Some(&"wallet".to_string()) {
        recv_wallet = Some(i + 2);
      }
    }

    let fee_amount = row.iter().position(|s| s == "fee");
    let mut fee_asset = pos(&row, &["fee asset", "f asset"]);
    if let Some(i) = fee_amount {
      if fee_asset.is_none() && row.get(i + 1) == Some(&"asset".to_string()) {
        fee_asset = Some(i + 1);
      }
    }

    let cost = row.iter().position(|s| s == "cost");

    Ok(CsvCols {
      kind,
      date,
      note,
      hash,
      sent_amount,
      sent_asset,
      sent_wallet,
      recv_amount,
      recv_asset,
      recv_wallet,
      fee_amount,
      fee_asset,
      cost,
    })
  }
  fn get_or_empty<'a>(
    &self,
    row: &'a StringRecord,
    col: Option<usize>,
    name: &str,
  ) -> Result<&'a str, String> {
    match col {
      Some(i) => row.get(i).ok_or(format!("Missing \"{}\" cell", name)),
      None => Ok(""),
    }
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

fn parse_local_datetime(text: &str, format: &str) -> Result<i64, String> {
  let naive_dt = match NaiveDateTime::parse_from_str(text, format) {
    Ok(d) => d,
    Err(e) => throw!("Invalid date: {}", e),
  };
  match Local.from_local_datetime(&naive_dt) {
    chrono::LocalResult::Single(d) => Ok(d.timestamp_millis()),
    _ => throw!("Unable to add timezone to date {}", naive_dt),
  }
}

fn parse_kind(kind: &str) -> &str {
  match kind {
    "Withdraw" => "Withdrawal",
    other => other,
  }
}

async fn from_csv_record(row: StringRecord, cols: &CsvCols) -> Result<UncostedTransaction, String> {
  let kind = cols.get(&row, cols.kind, "Kind")?;
  let date = cols.get(&row, cols.date, "Date")?;
  let note = cols.get(&row, cols.note, "Note")?;
  let hash = cols.get(&row, cols.hash, "Hash")?;
  let cost = cols.get_or_empty(&row, cols.cost, "Cost")?;

  let sent_amount = cols.get(&row, cols.sent_amount, "Sent Amount")?.into();
  let sent_asset = cols.get(&row, cols.sent_asset, "Sent Asset")?.into();
  let sent_wallet = cols.get(&row, cols.sent_wallet, "Sent Wallet")?.into();

  let recv_amount = cols.get(&row, cols.recv_amount, "Received Amount")?.into();
  let recv_asset = cols.get(&row, cols.recv_asset, "Received Asset")?.into();
  let recv_wallet = cols.get(&row, cols.recv_wallet, "Received Wallet")?.into();

  let fee_amount = cols.get(&row, cols.fee_amount, "Fee Amount")?.into();
  let fee_asset = cols.get(&row, cols.fee_asset, "Fee Asset")?.into();

  let base_transaction = BaseTransaction {
    tag: parse_kind(kind).into(),
    date: parse_local_datetime(date, "%Y-%m-%d %H:%M:%S")?,
    note: note.into(),
    hash: hash.into(),
    sent: Value::new(sent_amount, sent_asset, sent_wallet)?,
    recv: Value::new(recv_amount, recv_asset, recv_wallet)?,
    fee: Quantity::new(fee_amount, fee_asset)?,
    manual_worth: Quantity::parse(cost)?,
  };
  let uncosted_transaction = base_transaction.into_uncosted_transaction()?;

  println!("{:#?}", uncosted_transaction);
  Ok(uncosted_transaction)
}
