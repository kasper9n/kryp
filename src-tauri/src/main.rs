#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use data::{Data, Kryp};
use rust_decimal::{Decimal, RoundingStrategy};
use std::thread;
use tauri::api::{dialog, shell};
use tauri::async_runtime::Mutex;
use tauri::{
  command, window, AboutMetadata, AppHandle, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem,
  RunEvent, Submenu, Window, WindowEvent, WindowUrl,
};

mod calc;
mod data;
mod export;
mod fetch;
mod fetch_current;
mod holdings;
mod import;
mod prices;
mod reports;
mod tax;
mod transaction;

#[command]
fn error_popup(msg: String, win: Window) {
  println!("Error: {}", msg);
  thread::spawn(move || {
    dialog::message(Some(&win), "Error", msg);
  });
}

#[macro_export]
macro_rules! throw {
  ($($arg:tt)*) => {{
    return Err(format!($($arg)*))
  }};
}

#[macro_export]
macro_rules! err {
  ($($arg:tt)*) => {{
    Err(From::from(format!($($arg)*)))
  }};
}

pub fn round_8(num: Decimal) -> Decimal {
  return num.round_dp_with_strategy(8, RoundingStrategy::MidpointAwayFromZero);
}

fn main() {
  let ctx = tauri::generate_context!();
  let tauri_app = tauri::Builder::default()
    .manage(import::ImportData::default())
    .invoke_handler(tauri::generate_handler![
      error_popup,
      data::new_file,
      data::open,
      data::save,
      data::close,
      data::is_open,
      data::get_tax,
      data::get_tax_settings,
      data::get_transactions,
      data::add_transaction,
      holdings::get_holdings,
      holdings::get_holdings_valued,
      holdings::get_holdings_by_wallet,
      data::list_assets,
      data::get_prices,
      reports::get_report,
      import::scan_import_file,
      import::continue_import,
      import::cancel_import,
      export::export,
    ])
    .menu(Menu::with_items([
      #[cfg(target_os = "macos")]
      MenuEntry::Submenu(Submenu::new(
        &ctx.package_info().name,
        Menu::with_items([
          MenuItem::About(ctx.package_info().name.clone(), AboutMetadata::default()).into(),
          MenuItem::Separator.into(),
          CustomMenuItem::new("Preferences...", "Preferences...")
            .accelerator("cmdOrControl+,")
            .into(),
          MenuItem::Separator.into(),
          MenuItem::Services.into(),
          MenuItem::Separator.into(),
          MenuItem::Hide.into(),
          MenuItem::HideOthers.into(),
          MenuItem::ShowAll.into(),
          MenuItem::Separator.into(),
          MenuItem::Quit.into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "File",
        Menu::with_items([
          CustomMenuItem::new("New", "New")
            .accelerator("cmdOrControl+N")
            .into(),
          CustomMenuItem::new("Open...", "Open...")
            .accelerator("cmdOrControl+O")
            .into(),
          MenuItem::Separator.into(),
          CustomMenuItem::new("Save", "Save")
            .accelerator("cmdOrControl+S")
            .into(),
          CustomMenuItem::new("Save As...", "Save As...")
            .accelerator("shift+cmdOrControl+S")
            .into(),
          MenuItem::Separator.into(),
          CustomMenuItem::new("Import...", "Import...")
            .accelerator("cmdOrControl+I")
            .into(),
          CustomMenuItem::new("Export...", "Export...")
            .accelerator("cmdOrControl+E")
            .into(),
          CustomMenuItem::new("Close", "Close")
            .accelerator("cmdOrControl+W")
            .into(),
          #[cfg(not(target_os = "macos"))]
          MenuItem::Separator.into(),
          #[cfg(not(target_os = "macos"))]
          CustomMenuItem::new("Preferences...", "Options...")
            .accelerator("cmdOrControl+,")
            .into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Edit",
        Menu::with_items([
          MenuItem::Undo.into(),
          MenuItem::Redo.into(),
          MenuItem::Separator.into(),
          MenuItem::Cut.into(),
          MenuItem::Copy.into(),
          MenuItem::Paste.into(),
          #[cfg(not(target_os = "macos"))]
          MenuItem::Separator.into(),
          MenuItem::SelectAll.into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "View",
        Menu::with_items([
          CustomMenuItem::new("Dashboard", "Dashboard")
            .accelerator("cmdOrControl+1")
            .into(),
          CustomMenuItem::new("Transactions", "Transactions")
            .accelerator("cmdOrControl+2")
            .into(),
          CustomMenuItem::new("Reports", "Reports")
            .accelerator("cmdOrControl+3")
            .into(),
          MenuItem::Separator.into(),
          MenuItem::EnterFullScreen.into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Window",
        Menu::with_items([MenuItem::Minimize.into(), MenuItem::Zoom.into()]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Help",
        Menu::with_items([CustomMenuItem::new("Learn More", "Learn More").into()]),
      )),
    ]))
    .on_menu_event(|event| match event.menu_item_id() {
      "Learn More" => {
        let url = "https://github.com/probablykasper/kryp".to_string();
        shell::open(&event.window().shell_scope(), url, None).unwrap();
      }
      _ => {}
    })
    .build(ctx)
    .expect("error while running tauri app");

  tauri_app.manage(data::Data(Mutex::new(Kryp::new(tauri_app.app_handle()))));

  let _ = window::WindowBuilder::new(&tauri_app.app_handle(), "main", WindowUrl::default())
    .title("Kryp")
    .resizable(true)
    .decorations(true)
    .always_on_top(false)
    .inner_size(1050.0, 800.0)
    .min_inner_size(300.0, 200.0)
    .fullscreen(false)
    .build();

  tauri_app.run(app_run)
}

fn app_run(app_handle: &AppHandle, run_event: RunEvent) {
  match run_event {
    tauri::RunEvent::WindowEvent { label, event, .. } => {
      handle_window_event(app_handle, label, event)
    }
    _ => {}
  }
}

fn handle_window_event(app_handle: &AppHandle, label: String, event: WindowEvent) {
  match event {
    WindowEvent::CloseRequested { api, .. } => {
      if label == "main" {
        let st = app_handle.state::<Data>();
        let kryp = tauri::async_runtime::block_on(st.0.lock());
        if kryp.has_unsaved_changes() {
          api.prevent_close();
          let app_handle = app_handle.clone();
          let w = app_handle.get_window(&label).unwrap();
          let title = "You have unsaved changes or newly fetched prices. Close without saving?";
          let dialog_w = w.clone();
          dialog::confirm(Some(&dialog_w), title, "", move |res| {
            if res == true {
              w.close().unwrap();
            }
          });
        }
      }
    }
    _ => {}
  }
}

pub async fn confirm_async<S: AsRef<str>>(w: Window, title: S, msg: S) -> bool {
  let (sender, receiver) = tokio::sync::oneshot::channel();
  let title = title.as_ref().to_string();
  let msg = msg.as_ref().to_string();
  dialog::confirm(Some(&w), title, msg, move |res| {
    sender.send(res).unwrap();
  });
  receiver.await.unwrap_or(false)
}
