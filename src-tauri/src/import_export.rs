use crate::data::Data;
use crate::tax::Tax;
use crate::throw;
use crate::transaction::{Deposit, Trade, Transaction, Transfer, Withdrawal};
use chrono::NaiveDateTime;
use csv::StringRecord;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
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

#[command]
pub async fn import(win: Window, kryp: State<'_, Data>) -> Result<(), String> {
  let file_path = match pick_file(&win) {
    Some(p) => p,
    None => return Ok(()),
  };

  let delimiter = match file_path.extension().unwrap_or_default().to_str() {
    Some("csv") => b',',
    Some("tsv") => b'\t',
    _ => throw!("Unknown file extension"),
  };
  let mut reader = csv::ReaderBuilder::new()
    .delimiter(delimiter)
    .from_path(file_path)
    .map_err(|_| "Error opening file".to_string())?;

  let header = match reader.headers() {
    Ok(h) => h,
    Err(e) => throw!("Error reading file: {}", e),
  };
  let cols = CsvCols::from_header(header)?;
  println!("{:?}", cols);

  let mut kryp = kryp.0.lock().await;
  let mut n = 2; // 2 to account for header
  let mut transactions = Vec::new();
  for record in reader.records() {
    let row = record.map_err(|e| format!("Unable to read row {}: {}", n, e))?;
    let transaction = from_csv_record(row, &cols, &mut kryp.tax)
      .await
      .map_err(|e| format!("Error in row {}: {}", n, e))?;
    transactions.push(transaction);
    n += 1;
  }
  for transaction in transactions {
    kryp.tax.add_transaction(transaction);
  }
  kryp.tax.calculate()?;
  Ok(())
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
  let s = value.get(i).unwrap();
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
        println!("3");
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
    })
  }
}

async fn from_csv_record(
  row: StringRecord,
  cols: &CsvCols,
  tax: &mut Tax,
) -> Result<Transaction, String> {
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

  let format = "%Y-%m-%d %H:%M:%S";
  let date = match NaiveDateTime::parse_from_str(row.get(date_i).unwrap(), format) {
    Ok(d) => d,
    Err(e) => throw!("Invalid date: {}", e),
  };
  let kind = row.get(type_i).unwrap();
  let mut transaction = match kind {
    "Trade" => Transaction::Trade(Trade {
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

      manual_worth_amount: None,
      manual_worth_asset: None,
      cost: dec!(0),
    }),
    "Transfer" => Transaction::Transfer(Transfer {
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

      manual_worth_amount: None,
      manual_worth_asset: None,
      cost: dec!(0),
    }),
    "Deposit" | "Buy" | "Income" | "Gift" => {
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

      Transaction::Deposit(Deposit {
        tag: kind.to_string(),
        date: date.timestamp_millis(),
        note: row.get(note_i).unwrap().into(),
        hash: row.get(hash_i).unwrap().into(),

        from_amount,
        from_asset,

        amount: decimal_str(&row, recv_amount_i)?,
        asset: row.get(recv_asset_i).unwrap().into(),
        wallet: row.get(recv_wallet_i).unwrap().into(),

        cost: dec!(0),
      })
    }
    "Withdrawal" | "Sell" | "Spent" | "Lost" => {
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
      Transaction::Withdrawal(Withdrawal {
        tag: kind.to_string(),
        date: date.timestamp_millis(),
        note: row.get(note_i).unwrap().into(),
        hash: row.get(hash_i).unwrap().into(),

        amount: decimal_str(&row, sent_amount_i)?,
        asset: row.get(sent_asset_i).unwrap().into(),
        wallet: row.get(sent_wallet_i).unwrap().into(),

        to_amount,
        to_asset,

        cost: dec!(0),
      })
    }
    _ => throw!("Invalid type \"{}\"", kind),
  };
  println!("{:#?}", transaction);
  transaction
    .refresh_cost(&mut tax.price_data, &tax.settings.base_currency)
    .await?;
  Ok(transaction)
}
