use crate::data::Data;
use crate::throw;
use std::path::PathBuf;
use std::sync::mpsc;
use tauri::api::dialog;
use tauri::{command, State, Window};

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

#[command]
pub async fn export(win: Window, kryp: State<'_, Data>) -> Result<(), String> {
  let kryp = kryp.0.lock().await;
  if !kryp.is_open() {
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
    "Tx Hash", "Date", "Cost",
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
