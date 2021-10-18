use crate::tax::Tax;
use crate::throw;
use crate::transaction::{Transaction, TxType};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::api::dialog;
use tauri::{command, State, Window};

#[derive(Serialize, Deserialize, Debug)]
pub struct Kryp {
  tax: Tax,
  opened: bool,
  file_path: Option<PathBuf>,
}

impl Kryp {
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

impl Default for Kryp {
  fn default() -> Self {
    Kryp {
      tax: Tax::new("USD"),
      opened: false,
      file_path: None,
    }
  }
}

pub fn to_json<T: Serialize>(data: &T) -> Result<Value, String> {
  match serde_json::to_value(data) {
    Ok(v) => Ok(v),
    Err(e) => throw!("Error serializing {}", e),
  }
}

// Call whenever opened is updated
macro_rules! refresh_menu_bar {
  ($kryp:ident, $win:ident) => {{
    let menu_handle = $win.menu_handle();
    menu_handle
      .get_item(&"Save".to_string())
      .set_enabled($kryp.opened)
      .unwrap();
    menu_handle
      .get_item(&"Save As...".to_string())
      .set_enabled($kryp.opened)
      .unwrap();
  }};
}

#[derive(Default)]
pub struct Data(pub Arc<Mutex<Kryp>>);

pub type St<'a> = State<'a, Data>;

#[command]
pub async fn new_file(
  base_currency: String,
  kryp: State<'_, Data>,
  win: Window,
) -> Result<(), String> {
  let mut kryp = kryp.0.lock().unwrap();
  if kryp.opened == false {
    kryp.tax = Tax::new(&base_currency);
    kryp.opened = true;
    kryp.file_path = None;
    refresh_menu_bar!(kryp, win);
  }
  Ok(())
}

#[command]
pub async fn load_file(path: PathBuf, kryp: State<'_, Data>, win: Window) -> Result<(), String> {
  let mut kryp = kryp.0.lock().unwrap();
  if kryp.opened == false {
    println!("open file {:?}", path);
    kryp.tax = Tax::load(&path)?;
    kryp.opened = true;
    kryp.file_path = Some(path);
    refresh_menu_bar!(kryp, win);
  }
  Ok(())
}

#[command]
pub async fn open(path: Option<PathBuf>, kryp: State<'_, Data>, win: Window) -> Result<(), String> {
  let mut kryp = kryp.0.lock().unwrap();
  if kryp.opened == false {
    let file_path = match path {
      Some(path) => path,
      None => {
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut d = dialog::FileDialogBuilder::new().add_filter("Kryp", &["kryp"]);
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
    kryp.opened = true;
    kryp.file_path = Some(file_path);
    refresh_menu_bar!(kryp, win);
  }
  Ok(())
}

#[command]
pub async fn save(mut save_as: bool, kryp: State<'_, Data>) -> Result<(), String> {
  let kryp = kryp.0.lock().unwrap();
  if let None = kryp.file_path {
    save_as = true;
  }
  println!("save as {}", save_as);
  if save_as {
    let (sender, receiver) = std::sync::mpsc::channel();
    dialog::FileDialogBuilder::new()
      .set_file_name("report.kryp")
      .add_filter("Kryp", &["kryp"])
      .save_file(move |p| {
        sender.send(p).unwrap();
      });
    match receiver.recv().unwrap_or_default() {
      Some(file_path) => kryp.tax.save(file_path),
      None => return Ok(()),
    };
  }
  Ok(())
}

#[command]
/// Returns a hideApp bool
pub async fn close(kryp: State<'_, Data>, win: Window) -> Result<bool, String> {
  let mut kryp = kryp.0.lock().unwrap();
  if kryp.has_unsaved_changes() {
    let res = crate::dialog_sync(
      win.clone(),
      "You have unsaved changes. Close without saving?",
      "",
    );
    if res == false {
      return Ok(false);
    }
  }
  if !kryp.opened {
    win.close().unwrap();
    *kryp = Kryp::default();
    refresh_menu_bar!(kryp, win);
    return Ok(true);
  } else {
    *kryp = Kryp::default();
    refresh_menu_bar!(kryp, win);
    return Ok(false);
  }
}

#[command]
pub async fn get_data(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().unwrap();
  let v = serde_json::json!({
    "opened": kryp.opened,
  });
  return Ok(v);
}

#[command]
pub async fn get_tax(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().unwrap();
  to_json(&kryp.tax)
}

#[command]
pub async fn calculate(kryp: State<'_, Data>) -> Result<(), String> {
  let mut kryp = kryp.0.lock().unwrap();
  kryp.tax.calculate()?;
  Ok(())
}

#[command]
pub async fn get_transactions(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().unwrap();
  to_json(&kryp.tax.transactions)
}

#[command]
pub fn add_transaction(ttype: TxType, json: String, kryp: State<Data>) -> Result<(), String> {
  let mut kryp = kryp.0.lock().unwrap();
  let mut tx = Transaction::from_json(ttype, &json)?;
  let tax = &mut kryp.tax;
  let cost = tx.calculate_cost(&mut tax.price_data, &tax.base_currency);
  tx.set_cost(cost);
  kryp.tax.add_transaction(tx)?;
  Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Holding {
  key: String,
  amount: Decimal,
  cost: Decimal,
}

#[command]
pub async fn get_balances_by_asset(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().unwrap();
  let mut holdings_map: HashMap<String, Holding> = HashMap::new();
  for balance in &kryp.tax.balances {
    let key = balance.currency.clone();
    let holding = holdings_map.entry(key.clone()).or_insert(Holding {
      key,
      amount: dec!(0),
      cost: dec!(0),
    });
    holding.amount += balance.amount;
    holding.cost += balance.cost;
  }
  let mut holdings: Vec<Holding> = holdings_map.into_iter().map(|(_k, v)| v).collect();
  holdings.sort_by(|a, b| a.amount.cmp(&b.amount));

  to_json(&holdings)
}

#[command]
pub async fn get_prices(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().unwrap();
  to_json(&kryp.tax.price_data)
}
