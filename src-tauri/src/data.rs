use crate::import::ImportData;
use crate::prices::PriceDataAsset;
use crate::tax::{Tax, TaxSettings};
use crate::transaction::UncostedTransaction;
use crate::{confirm_async, throw};
use serde::Serialize;
use serde_json::Value;
use std::path::PathBuf;
use tauri::{command, AppHandle, Emitter, State, Window};
use tauri_plugin_dialog::{DialogExt, FilePath};
use tauri_specta::Event;
use tokio::sync::Mutex;

pub struct Kryp {
	pub tax: Tax,
	opened: bool,
	app: AppHandle,
	pub file_path: Option<PathBuf>,
	pub import_data: ImportData,
}

#[derive(Clone, Serialize, specta::Type, tauri_specta::Event)]
pub struct OpenedEvent {
	opened: bool,
	file_path: Option<PathBuf>,
}

impl Kryp {
	pub fn new(app: AppHandle) -> Self {
		Kryp {
			tax: Tax::new("USD"),
			opened: false,
			app,
			file_path: None,
			import_data: ImportData::default(),
		}
	}
	pub fn is_open(&self) -> bool {
		self.opened
	}
	pub fn emit_file_status(&mut self, opened: bool) {
		if self.opened != opened {
			self.opened = opened;
			if !opened {
				*self = Kryp::new(self.app.clone());
			}
		}
		println!("EMIT opened {}", opened);
		OpenedEvent {
			opened,
			file_path: self.file_path.clone(),
		}
		.emit(&self.app);
	}
	pub fn has_unsaved_changes(&self) -> bool {
		self.opened && self.tax.dirty
	}
	// pub fn set_opened<P: Params>(mut self, window: Window<P>, value: bool) {
	// 	self.opened = value;
	// 	let menu_handle = window.menu_handle();
	// 	// std::thread::spawn(move || {
	// 		let id: P::MenuId = "Save".into();
	// 		menu_handle.get_item("Save").set_enabled(value);
	// 	// });
	// }
}

pub fn to_json<T: Serialize>(data: &T) -> Result<Value, String> {
	match serde_json::to_value(data) {
		Ok(v) => Ok(v),
		Err(e) => throw!("Error serializing {}", e),
	}
}

pub struct Data(pub Mutex<Kryp>);

#[command]
#[specta::specta]
pub async fn new_file(base_currency: String, kryp: State<'_, Data>) -> Result<(), String> {
	let mut kryp = kryp.0.lock().await;
	if !kryp.is_open() {
		kryp.tax = Tax::new(&base_currency);
		kryp.file_path = None;
		kryp.emit_file_status(true);
	}
	Ok(())
}

#[command]
#[specta::specta]
pub async fn open(path: Option<PathBuf>, kryp: State<'_, Data>, win: Window) -> Result<(), String> {
	let mut kryp = kryp.0.lock().await;
	if !kryp.is_open() {
		let file_path = match path {
			Some(path) => path,
			None => {
				let d = win
					.dialog()
					.file()
					.add_filter("Kryp", &["json"])
					.set_parent(&win)
					.blocking_pick_file();
				match d {
					Some(file_path) => file_path.into_path().unwrap(),
					None => return Ok(()),
				}
			}
		};
		println!("open file {:?}", file_path);
		kryp.tax = Tax::load(file_path.clone())?;
		kryp.file_path = Some(file_path);
		kryp.emit_file_status(true);
	}
	Ok(())
}

#[command]
#[specta::specta]
pub async fn save(save_as: bool, kryp: State<'_, Data>) -> Result<(), String> {
	let mut kryp = kryp.0.lock().await;
	if !kryp.opened {
		return Ok(());
	}
	let mut save_path = &kryp.file_path;
	if save_as {
		save_path = &None;
	}
	println!("save as? {}", save_path.is_none());
	if let Some(path) = save_path {
		kryp.tax.save(path.clone());
		kryp.tax.dirty = false;
	} else {
		let file_path = kryp
			.app
			.dialog()
			.file()
			.set_file_name("Kryp Tax.json")
			.add_filter("Kryp", &["json"])
			.blocking_save_file();
		if let Some(file_path) = file_path {
			let file_path = file_path.into_path().unwrap();
			kryp.tax.save(file_path.clone());
			kryp.file_path = Some(file_path);
			kryp.tax.dirty = false;
			kryp.emit_file_status(true);
		}
	}
	Ok(())
}

#[command]
#[specta::specta]
/// Returns a hideApp bool
pub async fn close(kryp: State<'_, Data>, win: Window) -> Result<(), String> {
	let mut kryp = kryp.0.lock().await;
	if kryp.has_unsaved_changes() {
		let title = "You have unsaved changes or newly fetched prices. Close without saving?";
		let res = confirm_async(win.clone(), title, "");
		if res.await == false {
			return Ok(());
		}
	}
	if !kryp.opened {
		win.destroy().unwrap();
	}
	kryp.emit_file_status(false);
	Ok(())
}

#[command]
#[specta::specta]
pub async fn is_open(kryp: State<'_, Data>) -> Result<bool, String> {
	let kryp = kryp.0.lock().await;
	Ok(kryp.opened)
}

#[command]
#[specta::specta]
pub async fn get_tax(kryp: State<'_, Data>) -> Result<Tax, String> {
	let kryp = kryp.0.lock().await;
	Ok(kryp.tax.clone())
}

#[command]
#[specta::specta]
pub async fn get_tax_settings(kryp: State<'_, Data>) -> Result<TaxSettings, String> {
	let kryp = kryp.0.lock().await;
	Ok(kryp.tax.settings.clone())
}

#[command]
#[specta::specta]
pub async fn set_tax_settings(kryp: State<'_, Data>, settings: TaxSettings) -> Result<(), String> {
	let mut kryp = kryp.0.lock().await;
	kryp.tax.settings = settings;
	Ok(())
}

#[command]
#[specta::specta]
pub async fn add_transaction(
	uncosted_tx: UncostedTransaction,
	kryp: State<'_, Data>,
) -> Result<(), String> {
	let mut kryp = kryp.0.lock().await;
	let tax = &mut kryp.tax;
	let base = &tax.settings.base_currency;
	let tx = uncosted_tx
		.auto_cost_and_finalize(&mut tax.price_data, &tax.settings.apis, base)
		.await?;
	kryp.tax.add_transaction(tx);
	kryp.tax.calculate()?;
	Ok(())
}

#[command]
#[specta::specta]
pub async fn list_assets(kryp: State<'_, Data>) -> Result<Vec<String>, String> {
	let kryp = kryp.0.lock().await;
	let assets = kryp
		.tax
		.price_data
		.list_assets()
		.into_iter()
		.map(|a| a.to_string())
		.collect();
	Ok(assets)
}

#[command]
#[specta::specta]
pub async fn get_prices(symbol: String, kryp: State<'_, Data>) -> Result<PriceDataAsset, String> {
	let kryp = kryp.0.lock().await;
	let pd = &kryp.tax.price_data;
	let asset = pd.get_asset(&symbol).ok_or("Asset not found".to_string())?;
	Ok(asset.clone())
}
