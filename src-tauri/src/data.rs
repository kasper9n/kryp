use crate::fetch_current::fetch_current;
use crate::import_export::ImportData;
use crate::tax::Tax;
use crate::transaction::UncostedTransaction;
use crate::{confirm_async, throw};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::api::dialog;
use tauri::{command, State, Window};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct Kryp {
  pub tax: Tax,
  pub opened: bool,
  pub file_path: Option<PathBuf>,
  pub import_data: ImportData,
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
      import_data: ImportData::default(),
    }
  }
}

pub fn to_json<T: Serialize>(data: &T) -> Result<Value, String> {
  match serde_json::to_value(data) {
    Ok(v) => Ok(v),
    Err(e) => throw!("Error serializing {}", e),
  }
}

#[derive(Default)]
pub struct Data(pub Arc<Mutex<Kryp>>);

#[command]
pub async fn new_file(base_currency: String, kryp: State<'_, Data>) -> Result<(), String> {
  let mut kryp = kryp.0.lock().await;
  if kryp.opened == false {
    kryp.tax = Tax::new(&base_currency);
    kryp.opened = true;
    kryp.file_path = None;
  }
  Ok(())
}

#[command]
pub async fn open(path: Option<PathBuf>, kryp: State<'_, Data>, win: Window) -> Result<(), String> {
  let mut kryp = kryp.0.lock().await;
  if kryp.opened == false {
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
    kryp.opened = true;
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
pub async fn close(kryp: State<'_, Data>, win: Window) -> Result<bool, String> {
  let mut kryp = kryp.0.lock().await;
  if kryp.has_unsaved_changes() {
    let title = "You have unsaved changes or newly fetched prices. Close without saving?";
    let res = confirm_async(win.clone(), title, "");
    if res.await == false {
      return Ok(false);
    }
  }
  if !kryp.opened {
    win.close().unwrap();
    *kryp = Kryp::default();
    return Ok(true);
  } else {
    *kryp = Kryp::default();
    return Ok(false);
  }
}

#[command]
pub async fn get_data(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().await;
  let v = serde_json::json!({
    "opened": kryp.opened,
  });
  return Ok(v);
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
  let tx = UncostedTransaction::from_json(&json)?
    .auto_cost_and_finalize(&mut tax.price_data, &tax.settings.apis, base)
    .await?;
  kryp.tax.add_transaction(tx);
  kryp.tax.calculate()?;
  Ok(())
}

#[derive(Serialize, Debug)]
struct Holding {
  asset: String,
  amount: Decimal,
  cost: Decimal,
  value: Option<Decimal>,
  error: Option<String>,
}
impl Holding {
  fn new(key: String) -> Self {
    Self {
      asset: key,
      amount: dec!(0),
      cost: dec!(0),
      value: None,
      error: None,
    }
  }
}

fn get_holdings_unsorted(tax: &Tax) -> Vec<Holding> {
  let mut holdings_map: HashMap<String, Holding> = HashMap::new();
  for balance in &tax.balances {
    let key = balance.currency.clone();
    if balance.amount > dec!(0) {
      let holding = holdings_map.entry(key.clone()).or_insert(Holding::new(key));
      holding.amount += balance.amount;
      holding.cost += balance.cost;
    }
  }
  holdings_map.into_iter().map(|(_k, v)| v).collect()
}

#[command]
pub async fn get_holdings(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().await;
  let mut holdings = get_holdings_unsorted(&kryp.tax);
  holdings.sort_by(|a, b| a.amount.cmp(&b.amount));
  to_json(&holdings)
}

#[command]
pub async fn get_holdings_valued(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().await;
  let mut holdings = get_holdings_unsorted(&kryp.tax);

  let assets: Vec<_> = holdings.iter().map(|h| &h.asset).collect();
  let prices = fetch_current(assets, &kryp.tax.settings.base_currency).await?;

  for holding in &mut holdings {
    if let Some(price) = prices.get(&holding.asset) {
      holding.value = Some(holding.amount * price);
    } else {
      holding.error = Some("No price".to_string());
    }
  }
  holdings.sort_by(|a, b| b.value.cmp(&a.value));
  to_json(&holdings)
}

#[derive(Serialize, Debug)]
struct WalletHoldings {
  name: String,
  holdings: HashMap<String, Holding>,
}

#[command]
pub async fn get_holdings_by_wallet(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().await;
  let mut wallets_map: HashMap<String, WalletHoldings> = HashMap::new();
  for balance in &kryp.tax.balances {
    let asset = balance.currency.clone();
    if balance.amount > dec!(0) {
      let wallet = wallets_map
        .entry(balance.wallet.clone())
        .or_insert(WalletHoldings {
          name: balance.wallet.clone(),
          holdings: HashMap::new(),
        });
      let holding = wallet
        .holdings
        .entry(asset.clone())
        .or_insert(Holding::new(asset));
      holding.amount += balance.amount;
      holding.cost += balance.cost;
    }
  }
  to_json(&wallets_map)
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
