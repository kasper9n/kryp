use crate::data::Data;
use crate::throw;
use crate::transaction::{Deposit, Trade, Transfer, UncostedTransaction, Withdrawal};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use csv::StringRecord;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::mpsc;
use tauri::api::dialog;
use tauri::{command, State, Window};

fn pick_file(_win: &Window) -> Option<PathBuf> {
  let mut d = dialog::FileDialogBuilder::new().add_filter("Table", &["csv", "tsv"]);
  #[cfg(any(target_os = "macos", target_os = "windows"))]
  {
    d = d.set_parent(&_win);
  }
  let (sender, receiver) = mpsc::channel();
  d.pick_file(move |p| {
    sender.send(p).unwrap();
  });
  receiver.recv().unwrap_or_default()
}
fn save_file(_win: &Window) -> Option<PathBuf> {
  let mut d = dialog::FileDialogBuilder::new().add_filter("Table", &["csv", "tsv"]);
  #[cfg(any(target_os = "macos", target_os = "windows"))]
  {
    d = d.set_parent(&_win);
  }
  let (sender, receiver) = mpsc::channel();
  d.save_file(move |p| {
    sender.send(p).unwrap();
  });
  receiver.recv().unwrap_or_default()
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ImportData {
  transactions: Vec<ImportTransaction>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImportTransaction {
  pub transaction: UncostedTransaction,
  pub cost: Option<Decimal>,
  pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImportStatus {
  index: u64,
}

fn read_csv(file_path: PathBuf) -> Result<csv::Reader<File>, String> {
  let delimiter = match file_path.extension().unwrap_or_default().to_str() {
    Some("csv") => b',',
    Some("tsv") => b'\t',
    _ => throw!("Unknown file extension"),
  };
  let reader = csv::ReaderBuilder::new()
    .delimiter(delimiter)
    .from_path(file_path)
    .map_err(|_| "Error opening file".to_string())?;
  Ok(reader)
}

#[command]
pub async fn import(win: Window, kryp: State<'_, Data>) -> Result<Option<ImportData>, String> {
  let file_path = match pick_file(&win) {
    Some(p) => p,
    None => return Ok(None),
  };
  let mut reader = read_csv(file_path)?;

  let header = match reader.headers() {
    Ok(h) => h,
    Err(e) => throw!("Error reading file: {}", e),
  };
  let cols = CsvCols::from_header(header)?;
  println!("{:?}", cols);

  let mut kryp = kryp.0.lock().await;
  let tax = &mut kryp.tax;

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
    win.emit("importStatus", ImportStatus { index: i }).ok();
    uncosted_transactions.push(import_transaction);
    i += 1;
  }
  // let tax = &mut kryp.tax;
  // for uncosted_transaction in uncosted_transactions {
  //   tax.add_transaction(transaction);
  // }
  // kryp.tax.calculate()?;
  kryp.import_data = ImportData {
    transactions: uncosted_transactions.clone(),
  };
  Ok(Some(kryp.import_data.clone()))
}

#[command]
pub async fn export(win: Window, kryp: State<'_, Data>) -> Result<(), String> {
  let kryp = kryp.0.lock().await;
  if !kryp.opened {
    return Ok(());
  }
  let file_path = match save_file(&win) {
    Some(p) => p,
    None => return Ok(()),
  };

  let mut writer = match csv::Writer::from_path(file_path) {
    Ok(writer) => writer,
    Err(e) => throw!("Unable to write to file: {}", e),
  };
  let header_record = vec![
    "Type", "Sent", "Asset", "Wallet", "Received", "Asset", "Wallet", "Fee", "Asset", "Note",
    "Tx Hash", "Date",
  ];
  match writer.write_record(&header_record) {
    Ok(()) => {}
    Err(e) => throw!("Unable to write row: {}", e),
  };

  for transaction in &kryp.tax.transactions {
    let record = transaction.to_csv_record();
    match writer.write_record(&record) {
      Ok(()) => {}
      Err(e) => throw!("Unable to write row: {}", e),
    };
  }

  Ok(())
}

fn pos(row: &Vec<String>, values: &[&str]) -> Option<usize> {
  row.iter().position(|s| values.contains(&s.as_str()))
}

fn decimal_str(value: &StringRecord, i: usize) -> Result<Decimal, String> {
  let s = value.get(i).unwrap();
  match Decimal::from_str(s) {
    Ok(d) => Ok(d),
    Err(_) => throw!("Invalid number \"{}\" at column {}", s, i),
  }
}

fn optional_decimal_str(value: &StringRecord, i: usize) -> Result<Decimal, String> {
  let s = value.get(i).ok_or(format!("Missing cell {}", i))?;
  if s.trim() == "" {
    Ok(dec!(0))
  } else {
    decimal_str(value, i)
  }
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
}

fn parse_local_datetime(text: &str, format: &str) -> Result<DateTime<Local>, String> {
  let naive_dt = match NaiveDateTime::parse_from_str(text, format) {
    Ok(d) => d,
    Err(e) => throw!("Invalid date: {}", e),
  };
  match Local.from_local_datetime(&naive_dt) {
    chrono::LocalResult::Single(d) => Ok(d),
    _ => throw!("Unable to add timezone to date {}", naive_dt),
  }
}

fn parse_kind(kind: &str) -> &str {
  match kind {
    "Withdraw" => "Withdrawal",
    other => other,
  }
}

fn parse_cost(value: &str) -> Option<(Decimal, String)> {
  let mut chars = value.chars();
  let mut num_str = "".to_string();
  let mut period = false;
  loop {
    let c = chars.next()?;
    if c.is_ascii_digit() {
      num_str.push(c);
    } else if c == '.' && !period {
      num_str.push(c);
      period = true;
    } else {
      break;
    }
  }
  let num = Decimal::from_str(&num_str).ok()?;

  let asset_str: String = chars.collect();
  let asset = asset_str.trim().to_string();

  Some((num, asset))
}

async fn from_csv_record(row: StringRecord, cols: &CsvCols) -> Result<UncostedTransaction, String> {
  let type_i = cols.kind.ok_or("Missing \"Type\" column")?;
  let date_i = cols.date.ok_or("Missing \"Date\" column")?;
  let note_i = cols.note.ok_or("Missing \"Note\" column")?;
  let hash_i = cols.hash.ok_or("Missing \"Tx Hash\" column")?;

  let sent_amount_i = cols.sent_amount.ok_or("Missing \"Sent Amount\" column")?;
  let sent_asset_i = cols.sent_asset.ok_or("Missing \"Sent Asset\" column")?;
  let sent_wallet_i = cols.sent_wallet.ok_or("Missing \"Sent Wallet\" column")?;

  let recv_amount_i = cols.recv_amount.ok_or("Missing \"Received\" column")?;
  let recv_asset_i = cols.recv_asset.ok_or("Missing \"Received Asset\" column")?;
  let recv_wallet_i = cols
    .recv_wallet
    .ok_or("Missing \"Received Wallet\" column")?;

  let fee_amount_i = cols.fee_amount.ok_or("Missing \"Fee Amount\" column")?;
  let fee_asset_i = cols.fee_asset.ok_or("Missing \"Fee Asset\" column")?;

  let date = parse_local_datetime(row.get(date_i).unwrap(), "%Y-%m-%d %H:%M:%S")?;

  let kind = parse_kind(row.get(type_i).unwrap());

  let (cost_amount, cost_asset) = match cols.cost {
    Some(cost_i) => {
      let cell = row.get(cost_i).unwrap();
      if cell.trim() == "" {
        (None, None)
      } else {
        let cost = parse_cost(cell).ok_or(format!("Invalid cost cell: {}", cell))?;
        (Some(cost.0), Some(cost.1))
      }
    }
    None => (None, None),
  };

  let uncosted_transaction = match kind {
    "Trade" => UncostedTransaction::Trade(Trade {
      tag: kind.to_string(),
      date: date.timestamp_millis(),
      note: row.get(note_i).unwrap().into(),
      hash: row.get(hash_i).unwrap().into(),

      sent_amount: decimal_str(&row, sent_amount_i)?,
      sent_asset: row.get(sent_asset_i).unwrap().into(),
      sent_wallet: row.get(sent_wallet_i).unwrap().into(),

      recv_amount: decimal_str(&row, recv_amount_i)?,
      recv_asset: row.get(recv_asset_i).unwrap().into(),
      recv_wallet: row.get(recv_wallet_i).unwrap().into(),

      fee_amount: optional_decimal_str(&row, fee_amount_i)?,
      fee_asset: row.get(fee_asset_i).unwrap().into(),

      manual_worth_amount: cost_amount,
      manual_worth_asset: cost_asset,
      cost: dec!(0),
    }),
    "Transfer" => UncostedTransaction::Transfer(Transfer {
      tag: kind.to_string(),
      date: date.timestamp_millis(),
      note: row.get(note_i).unwrap().into(),
      hash: row.get(hash_i).unwrap().into(),

      sent_amount: decimal_str(&row, sent_amount_i)?,
      sent_asset: row.get(sent_asset_i).unwrap().into(),
      sent_wallet: row.get(sent_wallet_i).unwrap().into(),

      recv_amount: decimal_str(&row, recv_amount_i)?,
      recv_asset: row.get(recv_asset_i).unwrap().into(),
      recv_wallet: row.get(recv_wallet_i).unwrap().into(),

      manual_worth_amount: cost_amount,
      manual_worth_asset: cost_asset,
      cost: dec!(0),
    }),
    "Deposit" | "Buy" | "Income" | "Gift" | "Interest" => {
      let from_amount = match row.get(sent_amount_i).unwrap() {
        "" => None,
        _ => Some(decimal_str(&row, sent_amount_i)?),
      };
      let from_asset = match row.get(sent_asset_i).unwrap() {
        "" => None,
        s => Some(s.to_string()),
      };
      if (from_amount.is_some() || from_asset.is_some())
        && (from_amount.is_none() || from_asset.is_none())
      {
        throw!("The \"Sent\" columns are only partially filled in");
      }

      UncostedTransaction::Deposit(Deposit {
        tag: kind.to_string(),
        date: date.timestamp_millis(),
        note: row.get(note_i).unwrap().into(),
        hash: row.get(hash_i).unwrap().into(),

        from_amount: cost_amount,
        from_asset: cost_asset,

        amount: decimal_str(&row, recv_amount_i)?,
        asset: row.get(recv_asset_i).unwrap().into(),
        wallet: row.get(recv_wallet_i).unwrap().into(),

        cost: dec!(0),
      })
    }
    "Withdrawal" | "Sell" | "Spend" | "Lost" => {
      let to_amount = match row.get(recv_amount_i).unwrap() {
        "" => None,
        _ => Some(decimal_str(&row, recv_amount_i)?),
      };
      let to_asset = match row.get(recv_asset_i).unwrap() {
        "" => None,
        s => Some(s.to_string()),
      };
      if (to_amount.is_some() || to_asset.is_some()) && (to_amount.is_none() || to_asset.is_none())
      {
        throw!("The \"Sent\" columns are only partially filled in");
      }
      UncostedTransaction::Withdrawal(Withdrawal {
        tag: kind.to_string(),
        date: date.timestamp_millis(),
        note: row.get(note_i).unwrap().into(),
        hash: row.get(hash_i).unwrap().into(),

        amount: decimal_str(&row, sent_amount_i)?,
        asset: row.get(sent_asset_i).unwrap().into(),
        wallet: row.get(sent_wallet_i).unwrap().into(),

        to_amount: cost_amount,
        to_asset: cost_asset,

        cost: dec!(0),
      })
    }
    _ => throw!("Invalid type \"{}\"", kind),
  };
  println!("{:#?}", uncosted_transaction);
  Ok(uncosted_transaction)
}
