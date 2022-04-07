use crate::calc::Calculation;
use crate::data::Data;
use crate::tax::Tax;
use crate::throw;
use crate::transaction::{format_date, CoreTransaction, UncostedTransaction};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc;
use tauri::api::dialog;
use tauri::{command, State, Window};

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
  win: Window,
  kryp: State<'_, Data>,
) -> Result<Option<ImportData>, String> {
  let file_path = match pick_file(&win) {
    Some(p) => p,
    None => return Ok(None),
  };

  let mut kryp = kryp.0.lock().await;
  let tax = &mut kryp.tax;

  let import_data = match source.as_str() {
    "kryp" => kryp::read(read_csv(file_path)?, win, tax).await?,
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
