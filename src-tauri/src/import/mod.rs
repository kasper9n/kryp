use crate::calc::Calculation;
use crate::data::Data;
use crate::tax::Tax;
use crate::throw;
use crate::transaction::UncostedTransaction;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tauri::{command, Emitter, State, Window};
use tauri_plugin_dialog::{DialogExt, FilePath};

mod binance;
mod csv;
mod kryp;

#[derive(Serialize, Deserialize, Default, Clone, Debug, specta::Type)]
pub struct ImportData {
	transactions: Vec<ImportTransaction>,
	has_errors: bool,
	source: String,
}
impl ImportData {
	pub fn new(source: &str, transactions: Vec<ImportTransaction>) -> ImportData {
		let mut has_errors = false;
		for transaction in &transactions {
			if transaction.error.is_some() {
				has_errors = true;
			}
		}
		ImportData {
			transactions,
			has_errors,
			source: source.to_string(),
		}
	}
}

#[command]
#[specta::specta]
pub async fn get_import_data(kryp: State<'_, Data>) -> Result<ImportData, String> {
	let kryp = kryp.0.lock().await;
	Ok(kryp.import_data.clone())
}

#[command]
#[specta::specta]
pub async fn update_import_transactions(
	transactions: Vec<ImportTransaction>,
	kryp: State<'_, Data>,
) -> Result<ImportData, String> {
	let mut kryp = kryp.0.lock().await;
	let tax = &mut kryp.tax;

	let mut uncosted_transactions = Vec::new();
	for tx in transactions {
		let import_tx = ImportTransaction::from_uncosted_tx(tx.transaction, tax).await;
		uncosted_transactions.push(import_tx);
	}
	kryp.import_data = ImportData::new(&kryp.import_data.source, uncosted_transactions);
	Ok(kryp.import_data.clone())
}

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
pub struct ImportTransaction {
	pub transaction: UncostedTransaction,
	pub cost: Option<Decimal>,
	pub error: Option<String>,
}
impl ImportTransaction {
	pub async fn from_uncosted_tx(tx: UncostedTransaction, tax: &mut Tax) -> ImportTransaction {
		let cost = tx.get_or_calculate_cost(
			&mut tax.price_data,
			&tax.settings.apis,
			&tax.settings.base_currency,
		);
		match cost.await {
			Ok(cost) => ImportTransaction {
				transaction: tx,
				cost: Some(cost),
				error: None,
			},
			Err(e) => ImportTransaction {
				transaction: tx,
				cost: None,
				error: Some(e),
			},
		}
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImportStatus {
	index: usize,
	count: usize,
}

fn pick_files(_win: &Window) -> Option<Vec<FilePath>> {
	_win.dialog()
		.file()
		.add_filter("Table", &["csv", "tsv"])
		.set_parent(&_win)
		.blocking_pick_files()
}

/// Returns `true` if the scan was cancelled
#[command]
#[specta::specta]
pub async fn scan_import_file(
	source: String,
	tz: String,
	win: Window,
	kryp: State<'_, Data>,
) -> Result<bool, String> {
	let file_paths = match pick_files(&win) {
		Some(p) => p,
		None => return Ok(true),
	};
	let tz: chrono_tz::Tz = match tz.parse() {
		Ok(tz) => tz,
		Err(e) => throw!("Invalid timezone: {}", e),
	};

	let mut kryp = kryp.0.lock().await;
	let tax = &mut kryp.tax;

	let mut uncosted_transactions = Vec::new();
	for file_path in file_paths {
		let file_path = file_path.into_path().unwrap();
		let file_name = file_path.file_name().unwrap_or_default().to_owned();
		let result = match source.as_str() {
			"Kryp" => kryp::read(file_path, tz).await,
			"Binance" => binance::read(file_path).await,
			_ => throw!("Unsupported source: {}", source),
		};
		match result {
			Ok(mut transactions) => uncosted_transactions.append(&mut transactions),
			Err(e) => {
				throw!("Error in file {}\n\n{}", file_name.to_string_lossy(), e)
			}
		}
	}
	let transaction_count = uncosted_transactions.len();

	let mut import_transactions = Vec::new();
	for (i, uncosted_transaction) in uncosted_transactions.into_iter().enumerate() {
		let import_tx = ImportTransaction::from_uncosted_tx(uncosted_transaction, tax).await;
		import_transactions.push(import_tx);

		let status = ImportStatus {
			index: i,
			count: transaction_count,
		};
		win.emit("importStatus", status).ok();
	}

	kryp.import_data = ImportData::new(&source, import_transactions);

	println!("scan_import_file() done");
	Ok(false)
}

#[command]
#[specta::specta]
pub async fn cancel_import(kryp: State<'_, Data>) -> Result<(), ()> {
	let mut kryp = kryp.0.lock().await;
	kryp.import_data = ImportData::default();
	println!("cancel_import() done");
	Ok(())
}

#[command]
#[specta::specta]
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

	println!("continue_import() done");
	Ok(())
}
