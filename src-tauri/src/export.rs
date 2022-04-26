use crate::data::Data;
use crate::{save_csv_tsv, throw};
use tauri::{command, State, Window};

#[command]
pub async fn export(win: Window, kryp: State<'_, Data>) -> Result<(), String> {
  let kryp = kryp.0.lock().await;
  if !kryp.is_open() {
    return Ok(());
  }
  let file_name = format!("Kryp Export");
  let file_path = match save_csv_tsv(&win, &file_name) {
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
