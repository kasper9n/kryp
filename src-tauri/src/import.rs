use std::sync::mpsc;
use tauri::api::dialog;
use tauri::{command, Window};

#[command]
pub async fn import(win: Window) -> Result<(), String> {
  let mut d = dialog::FileDialogBuilder::new().add_filter("Table", &["csv", "tsv"]);
  #[cfg(any(target_os = "macos", target_os = "windows"))]
  {
    d = d.set_parent(&win);
  }
  let (sender, receiver) = mpsc::channel();
  d.pick_file(move |p| {
    sender.send(p).unwrap();
  });
  let file_path = match receiver.recv().unwrap_or_default() {
    Some(p) => p,
    None => return Ok(()),
  };
  println!("file_path {:?}", file_path);
  Ok(())
}
