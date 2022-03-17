use crate::calc::Calculation;
use crate::data::Data;
use crate::tax::Tax;
use crate::throw;
use crate::transaction::{
  format_date, CoreTransaction, Deposit, Quantity, Trade, Transfer, UncostedTransaction, Value,
  Withdrawal,
};
use chrono::{Local, NaiveDateTime, TimeZone};
use csv::StringRecord;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
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
  has_errors: bool,
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
pub async fn cancel_import(kryp: State<'_, Data>) -> Result<(), ()> {
  let mut kryp = kryp.0.lock().await;
  kryp.import_data = ImportData::default();
  Ok(())
}

#[command]
pub async fn start_import(
  win: Window,
  kryp: State<'_, Data>,
) -> Result<Option<ImportData>, String> {
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
  kryp.import_data = ImportData {
    transactions: uncosted_transactions.clone(),
    has_errors,
  };
  Ok(Some(kryp.import_data.clone()))
}

#[command]
pub async fn continue_import(kryp: State<'_, Data>) -> Result<(), String> {
  let mut kryp = kryp.0.lock().await;

  let mut transactions = Vec::new();
  for (i, uncosted_transaction) in kryp.import_data.transactions.iter().enumerate() {
    let cost = uncosted_transaction.cost.ok_or_else(|| {
      let date = uncosted_transaction.transaction.date();
      format!("Unable to get cost for transaction {} at {}", i, date)
    })?;
    let tx = uncosted_transaction.transaction.clone().finalize(cost);
    transactions.push(tx);
  }

  let mut new_transactions = kryp.tax.transactions.clone();
  for transaction in transactions {
    Tax::add_transaction_to_vec(&mut new_transactions, transaction);
  }

  let mut balances: HashMap<(String, String), Decimal> = HashMap::new();
  for tx in &new_transactions {
    let core_tx = CoreTransaction::from_transaction(tx.clone());
    let sent_amount = core_tx.sent_amount.unwrap_or(dec!(0));
    let sent_asset = core_tx.sent_asset.unwrap_or("".into());
    let sent_wallet = core_tx.sent_wallet.unwrap_or("".into());

    let recv_amount = core_tx.recv_amount.unwrap_or(dec!(0));
    let recv_asset = core_tx.recv_asset.unwrap_or("".into());
    let recv_wallet = core_tx.recv_wallet.unwrap_or("".into());

    let fee_amount = core_tx.fee_amount.unwrap_or(dec!(0));
    let fee_asset = core_tx.fee_asset.unwrap_or("".into());

    let recv_balance = balances.entry((recv_asset, recv_wallet)).or_insert(dec!(0));
    *recv_balance += recv_amount;

    {
      let sent_balance = balances
        .entry((sent_asset.clone(), sent_wallet.clone()))
        .or_insert(dec!(0));
      *sent_balance -= sent_amount;
      if sent_balance < &mut dec!(0) {
        let negative_balance = sent_balance.clone();
        println!("{:#?}", balances);
        throw!(
          "Negative balance {} {} in \"{}\" due to {} transaction at {}",
          negative_balance,
          sent_asset,
          sent_wallet,
          tx.tag(),
          format_date(tx.date()),
        );
      }
    }

    let mut fee_balance = balances
      .entry((fee_asset.clone(), sent_wallet.clone()))
      .or_insert(dec!(0));
    fee_balance -= fee_amount;
    if fee_balance < &mut dec!(0) {
      let negative_balance = fee_balance.clone();
      println!("{:?}", balances);
      throw!(
        "Negative balance {} {} in \"{}\" due to {} transaction at {}",
        negative_balance,
        fee_asset,
        sent_wallet,
        tx.tag(),
        format_date(tx.date()),
      );
    }
  }
  println!("SUCCESSx");
  for ((asset, wallet), amount) in balances {
    if amount != dec!(0) {
      println!("{} {} {}", wallet, asset, amount);
    }
  }

  let calculation = Calculation::calculate(&new_transactions)?;

  kryp.tax.transactions = new_transactions;
  kryp.tax.apply_calc_output(calculation);
  kryp.import_data = ImportData::default();

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

struct BaseTransaction {
  tag: String,
  date: i64,
  note: String,
  hash: String,
  sent: Option<Value>,
  recv: Option<Value>,
  fee: Option<Quantity>,
  manual_worth: Option<Quantity>,
}
impl BaseTransaction {
  fn into_uncosted_transaction(self) -> Result<UncostedTransaction, String> {
    let manual_worth_str = self.manual_worth.map(|q| q.to_string());
    let uncosted_transaction = match self.tag.as_str() {
      "Trade" => {
        let sent = self
          .sent
          .ok_or(format!("Sent amount is missing from {}", self.tag))?;
        let recv = self
          .recv
          .ok_or(format!("Received amount is missing from {}", self.tag))?;
        let fee = self.fee.unwrap_or(Quantity {
          amount: dec!(0),
          asset: "".into(),
        });
        UncostedTransaction::Trade(Trade {
          tag: self.tag,
          date: self.date,
          note: self.note,
          hash: self.hash,

          sent_amount: sent.amount,
          sent_asset: sent.asset,
          sent_wallet: sent.wallet,

          recv_amount: recv.amount,
          recv_asset: recv.asset,
          recv_wallet: recv.wallet,

          fee_amount: fee.amount,
          fee_asset: fee.asset,

          manual_worth: manual_worth_str,
          cost: dec!(0),
        })
      }
      "Transfer" => {
        let sent = self
          .sent
          .ok_or(format!("Sent amount is missing from {}", self.tag))?;
        let recv = self
          .recv
          .ok_or(format!("Received amount is missing from {}", self.tag))?;
        if self.fee.is_some() {
          throw!("Fee is not allowed for {}", self.tag);
        }
        UncostedTransaction::Transfer(Transfer {
          tag: self.tag,
          date: self.date,
          note: self.note,
          hash: self.hash,

          sent_amount: sent.amount,
          sent_asset: sent.asset,
          sent_wallet: sent.wallet,

          recv_amount: recv.amount,
          recv_asset: recv.asset,
          recv_wallet: recv.wallet,

          manual_worth: manual_worth_str,
          cost: dec!(0),
        })
      }
      "Deposit" | "Buy" | "Income" | "Gift" | "Interest" => {
        if self.sent.is_some() {
          throw!("Sent amount is not allowed for {}", self.tag);
        }
        let recv = self
          .recv
          .ok_or(format!("Received amount is missing from {}", self.tag))?;
        if self.fee.is_some() {
          throw!("Fee is not allowed for {}", self.tag);
        }
        UncostedTransaction::Deposit(Deposit {
          tag: self.tag,
          date: self.date,
          note: self.note,
          hash: self.hash,

          amount: recv.amount,
          asset: recv.asset,
          wallet: recv.wallet,

          manual_worth: manual_worth_str,
          cost: dec!(0),
        })
      }
      "Withdrawal" | "Sell" | "Spend" | "Lost" => {
        let sent = self
          .sent
          .ok_or(format!("Sent amount is missing from {}", self.tag))?;
        if self.recv.is_some() {
          throw!("Received amount is not allowed for {}", self.tag);
        }
        if self.fee.is_some() {
          throw!("Fee is not allowed for {}", self.tag);
        }
        UncostedTransaction::Withdrawal(Withdrawal {
          tag: self.tag,
          date: self.date,
          note: self.note,
          hash: self.hash,

          amount: sent.amount,
          asset: sent.asset,
          wallet: sent.wallet,

          manual_worth: manual_worth_str,
          cost: dec!(0),
        })
      }
      _ => throw!("Invalid type \"{}\"", self.tag),
    };
    Ok(uncosted_transaction)
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
  // pub fn kind<'a>(&self, row: &'a StringRecord) -> Result<&'a str, String> {
  //   self.get(row, self.kind, "Type")
  // }
  // pub fn date<'a>(&self, row: &'a StringRecord) -> Result<&'a str, String> {
  //   self.get(row, self.date, "Date")
  // }
  // pub fn sent(&self, row: &StringRecord) -> Result<AmountCells, String> {
  //   Ok(AmountCells {
  //     amount: self.get(&row, self.sent_amount, "Sent Amount")?.into(),
  //     asset: self.get(&row, self.sent_asset, "Sent Asset")?.into(),
  //     wallet: self.get(&row, self.sent_wallet, "Sent Wallet")?.into(),
  //   })
  // }
  // pub fn recv(&self, row: &StringRecord) -> Result<AmountCells, String> {
  //   Ok(AmountCells {
  //     amount: self.get(&row, self.recv_amount, "Received Amount")?.into(),
  //     asset: self.get(&row, self.recv_asset, "Received Asset")?.into(),
  //     wallet: self.get(&row, self.recv_wallet, "Received Wallet")?.into(),
  //   })
  // }
  // pub fn fee(&self, row: &StringRecord) -> Result<NumberCells, String> {
  //   Ok(NumberCells {
  //     amount: self.get(&row, self.fee_amount, "Fee Amount")?.into(),
  //     asset: self.get(&row, self.fee_asset, "Fee Asset")?.into(),
  //   })
  // }
  // pub fn cost(&self, row: &StringRecord) -> Result<(), String> {
  //   let cost_cell = self.get_or_empty(row, self.date, "Date")?;
  //   let (cost_amount, cost_asset) = match cost_cell.trim() {
  //     "" => (None, None),
  //     cell => {
  //       let cost = parse_quantity(cell).ok_or(format!("Invalid cost cell: {}", cell))?;
  //       (Some(cost.0), Some(cost.1))
  //     }
  //   };
  // }
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
