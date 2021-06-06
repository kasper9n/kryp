use crate::tax::Tax;
use crate::throw;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::api::dialog;
use tauri::{command, State};

#[derive(Debug, Default)]
pub struct Kryp {
  current: Option<Tax>,
  file_path: Option<PathBuf>,
}

impl Kryp {
  pub fn get_tax(&self) -> Result<&Tax, String> {
    match &self.current {
      None => throw!("No file is open"),
      Some(tax) => return Ok(tax),
    };
  }
}

#[derive(Default)]
pub struct Data(pub Arc<Mutex<Kryp>>);

#[command]
pub async fn get(kryp: State<'_, Data>) -> Result<Value, String> {
  let kryp = kryp.0.lock().unwrap();
  let tax = kryp.get_tax()?;
  let v = serde_json::to_value(&tax.transactions);
  match v {
    Ok(v) => return Ok(v),
    Err(e) => throw!("Error serializing {}", e),
  };
}

#[command]
pub async fn load_file(path: PathBuf, kryp: State<'_, Data>) -> Result<(), String> {
  let mut kryp = kryp.0.lock().unwrap();
  if let None = kryp.current {
    println!("open file {:?}", path);
    kryp.current = Some(Tax::load(&path)?);
    kryp.file_path = Some(path);
  }
  Ok(())
}

#[command]
pub async fn open(kryp: State<'_, Data>) -> Result<(), String> {
  let mut kryp = kryp.0.lock().unwrap();
  if let None = kryp.current {
    let path = dialog::FileDialogBuilder::new()
      .add_filter("Kryp", &["kryp"])
      .pick_file();
    let path = match path {
      Some(file_path) => file_path,
      None => return Ok(()),
    };
    println!("open file {:?}", path);
    kryp.current = Some(Tax::load(&path)?);
    kryp.file_path = Some(path);
  }
  Ok(())
}

#[command]
pub async fn save(mut save_as: bool, kryp: State<'_, Data>) -> Result<(), String> {
  let kryp = kryp.0.lock().unwrap();
  let tax = kryp.get_tax()?;
  if let None = kryp.file_path {
    save_as = true;
  }
  println!("save as {}", save_as);
  if save_as {
    let file_path = dialog::FileDialogBuilder::new()
      .set_file_name("report.kryp")
      .save_file();
    let file_path = match file_path {
      Some(file_path) => file_path,
      None => return Ok(()),
    };
    tax.save(file_path);
  }
  Ok(())
}
