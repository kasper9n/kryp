use crate::tax::Tax;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::api::dialog;
use tauri::{command, State};

#[macro_export]
macro_rules! throw {
  ($($arg:tt)*) => {{
    return Err(format!($($arg)*).to_owned())
  }};
}

#[derive(Debug, Default)]
pub struct Kryp {
  current: Option<Tax>,
  file_path: Option<PathBuf>,
}

#[derive(Default)]
pub struct Data(pub Arc<Mutex<Kryp>>);

#[command]
pub async fn open(kryp: State<'_, Data>) -> Result<(), String> {
  let mut kryp = kryp.0.lock().unwrap();
  if let None = kryp.current {
    let path = dialog::FileDialogBuilder::new()
      .add_filter("Kryp", &["krypj"])
      .add_filter("Kryp", &["krypj2"])
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
pub async fn save(mut save_as: bool, kryp: State<'_, Data>) -> Result<(), &str> {
  let kryp = kryp.0.lock().unwrap();
  let tax = match &kryp.current {
    None => return Err("No file is open"),
    Some(tax) => tax,
  };
  if let None = kryp.file_path {
    save_as = true;
  }
  println!("{}", save_as);
  if save_as {
    let file_path = dialog::FileDialogBuilder::new()
      .set_file_name("report.krypj")
      .add_filter("One", &["krypj"])
      .add_filter("One", &["krypj2"])
      .save_file();
    let file_path = match file_path {
      Some(file_path) => file_path,
      None => return Ok(()),
    };
    tax.save(file_path);
  }
  Ok(())
}
