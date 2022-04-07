use crate::import::ImportData;
use crate::tax::Tax;
use crate::transaction::UncostedTransaction;
use crate::{confirm_async, throw};
use serde::Serialize;
use serde_json::Value;
use std::path::PathBuf;
use tauri::api::dialog;
use tauri::{command, AppHandle, Manager, State, Window};
use tokio::sync::Mutex;

pub struct Kryp {
  pub tax: Tax,
  opened: bool,
  app: AppHandle,
  pub file_path: Option<PathBuf>,
  pub import_data: ImportData,
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
  pub fn set_opened(&mut self, opened: bool) {
    if self.opened != opened {
      self.opened = opened;
      if !opened {
        *self = Kryp::new(self.app.clone());
      }
      println!("EMIT opened");
      self.app.emit_all("opened", opened).unwrap();
    }
  }
  pub fn has_unsaved_changes(&self) -> bool {
    self.opened && self.tax.dirty
  }
  // pub fn set_opened<P: Params>(mut self, window: Window<P>, value: bool) {
  //   self.opened = value;
  //   let menu_handle = window.menu_handle();
  //   // std::thread::spawn(move || {
  //     let id: P::MenuId = "Save".into();
  //     menu_handle.get_item("Save").set_enabled(value);
  //   // });
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
pub async fn new_file(base_currency: String, kryp: State<'_, Data>) -> Result<(), String> {
  let mut kryp = kryp.0.lock().await;
  if !kryp.is_open() {
    kryp.tax = Tax::new(&base_currency);
    kryp.set_opened(true);
    kryp.file_path = None;
  }
  Ok(())
}

#[command]
pub async fn open(path: Option<PathBuf>, kryp: State<'_, Data>, win: Window) -> Result<(), String> {
  let mut kryp = kryp.0.lock().await;
  if !kryp.is_open() {
    let file_path = match path {
      Some(path) => path,
      None => {
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut d = dialog::FileDialogBuilder::new().add_filter("Kryp", &["json"]);
        if cfg!(any(target_os = "macos", target_os = "windows")) {
          d = d.set_parent(&win);
        }
        d.pick_file(move |p| {
          sender.send(p).unwrap();
        });
        match receiver.recv().unwrap_or_default() {
          Some(file_path) => file_path,
          None => return Ok(()),
        }
      }
    };
    println!("open file {:?}", file_path);
    kryp.tax = Tax::load(&file_path)?;
    kryp.set_opened(true);
    kryp.file_path = Some(file_path);
  }
  Ok(())
}

#[command]
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
    kryp.tax.save(path);
    kryp.tax.dirty = false;
  } else {
    let (sender, receiver) = std::sync::mpsc::channel();
    dialog::FileDialogBuilder::new()
      .set_file_name("Kryp Report.json")
      .add_filter("Kryp", &["json"])
      .save_file(move |p| {
        sender.send(p).unwrap();
      });
    match receiver.recv().unwrap_or_default() {
      Some(file_path) => {
        kryp.tax.save(&file_path);
        kryp.file_path = Some(file_path);
        kryp.tax.dirty = false;
      }
      None => return Ok(()),
    };
  }
  Ok(())
}

#[command]
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
    win.close().unwrap();
  }
  kryp.set_opened(false);
  Ok(())
}

#[command]
pub async fn is_open(kryp: State<'_, Data>) -> Result<bool, String> {
  let kryp = kryp.0.lock().await;
  Ok(kryp.opened)
}

#[command]
pub async fn get_tax(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().await;
  to_json(&kryp.tax)
}

#[command]
pub async fn get_tax_settings(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().await;
  to_json(&kryp.tax.settings)
}

#[command]
pub async fn get_transactions(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().await;
  to_json(&kryp.tax.transactions)
}

#[command]
pub async fn add_transaction(json: String, kryp: State<'_, Data>) -> Result<(), String> {
  let mut kryp = kryp.0.lock().await;
  let tax = &mut kryp.tax;
  let base = &tax.settings.base_currency;
  let uncosted_tx = UncostedTransaction::from_json(&json)?;
  let tx = uncosted_tx
    .auto_cost_and_finalize(&mut tax.price_data, &tax.settings.apis, base)
    .await?;
  kryp.tax.add_transaction(tx);
  kryp.tax.calculate()?;
  Ok(())
}

#[command]
pub async fn list_assets(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().await;
  let assets = &kryp.tax.price_data.list_assets();
  to_json(assets)
}

#[command]
pub async fn get_prices(symbol: String, kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().await;
  let pd = &kryp.tax.price_data;
  let asset = pd.get_asset(&symbol).ok_or("Asset not found".to_string())?;
  to_json(&asset)
}
