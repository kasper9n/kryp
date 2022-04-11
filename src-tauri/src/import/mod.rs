use crate::calc::Calculation;
use crate::data::Data;
use crate::tax::Tax;
use crate::throw;
use crate::transaction::UncostedTransaction;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc;
use tauri::api::dialog;
use tauri::{command, State, Window};

mod binance;
mod kryp;

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

#[command]
pub async fn scan_import_file(
  source: String,
  tz: String,
  win: Window,
  kryp: State<'_, Data>,
) -> Result<Option<ImportData>, String> {
  let file_path = match pick_file(&win) {
    Some(p) => p,
    None => return Ok(None),
  };
  let tz: chrono_tz::Tz = match tz.parse() {
    Ok(tz) => tz,
    Err(e) => throw!("Invalid timezone: {}", e),
  };

  let mut kryp = kryp.0.lock().await;
  let tax = &mut kryp.tax;

  let import_data = match source.as_str() {
    "Kryp" => kryp::read(read_csv(file_path)?, tz, win, tax).await?,
    "Binance" => binance::read(read_csv(file_path)?, win, tax).await?,
    _ => throw!("Unsupported source: {}", source),
  };

  kryp.import_data = import_data.clone();
  Ok(Some(import_data))
}

#[command]
pub async fn cancel_import(kryp: State<'_, Data>) -> Result<(), ()> {
  let mut kryp = kryp.0.lock().await;
  kryp.import_data = ImportData::default();
  Ok(())
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

  let calculation = Calculation::calculate(new_transactions.iter().collect())?;

  kryp.tax.transactions = new_transactions;
  kryp.tax.apply_calc_output(calculation);
  kryp.import_data = ImportData::default();

  Ok(())
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

pub fn get_cell_index(row: &Vec<String>, string: &[&str]) -> Result<usize, String> {
  match row.iter().position(|s| string.contains(&s.as_str())) {
    Some(i) => Ok(i),
    None => throw!("Missing column \"{}\"", string.get(0).unwrap_or(&"None")),
  }
}
