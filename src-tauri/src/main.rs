#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::data::St;
use rust_decimal::{Decimal, RoundingStrategy};
use std::thread;
use tauri::api::{dialog, shell};
use tauri::{command, CustomMenuItem, Manager, Menu, MenuItem, Window, WindowBuilder, WindowUrl};

mod data;
mod prices;
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

pub fn round_8(num: Decimal) -> Decimal {
  return num.round_dp_with_strategy(8, RoundingStrategy::MidpointAwayFromZero);
}

fn custom_item(name: &str) -> CustomMenuItem {
  CustomMenuItem::new(name.to_string(), name)
}

fn main() {
  let ctx = tauri::generate_context!();
  let tauri_app = tauri::Builder::default()
    .create_window("main", WindowUrl::default(), |win, webview| {
      let win = win
        .title("Kryp")
        .resizable(true)
        .transparent(false)
        .decorations(true)
        .always_on_top(false)
        .inner_size(1050.0, 800.0)
        .min_inner_size(300.0, 200.0)
        .fullscreen(false);
      return (win, webview);
    })
    .manage(data::Data(Default::default()))
    .invoke_handler(tauri::generate_handler![
      error_popup,
      data::new_file,
      data::load_file,
      data::open,
      data::save,
      data::close,
      data::export,
      data::get_data,
      data::get_tax,
      data::get_transactions,
      data::add_transaction,
      data::get_holdings,
      data::get_prices,
    ])
    .menu(Menu::with_items([
      #[cfg(target_os = "macos")]
      MenuItem::new_submenu(
        &ctx.package_info().name,
        [
          MenuItem::About(ctx.package_info().name.clone()),
          MenuItem::Separator,
          MenuItem::Services,
          MenuItem::Separator,
          MenuItem::Hide,
          MenuItem::HideOthers,
          MenuItem::ShowAll,
          MenuItem::Separator,
          MenuItem::Quit,
        ],
      ),
      MenuItem::new_submenu(
        "File",
        [
          custom_item("New").accelerator("cmdOrControl+N").into(),
          custom_item("Open...").accelerator("cmdOrControl+O").into(),
          MenuItem::Separator,
          custom_item("Save").accelerator("cmdOrControl+S").into(),
          custom_item("Save As...")
            .accelerator("shift+cmdOrControl+S")
            .into(),
          MenuItem::Separator,
          custom_item("Export...")
            .accelerator("cmdOrControl+E")
            .into(),
          custom_item("Close").accelerator("cmdOrControl+W").into(),
        ],
      ),
      MenuItem::new_submenu(
        "Edit",
        [
          MenuItem::Undo,
          MenuItem::Redo,
          MenuItem::Separator,
          MenuItem::Cut,
          MenuItem::Copy,
          MenuItem::Paste,
          #[cfg(not(target_os = "macos"))]
          MenuItem::Separator,
          MenuItem::SelectAll,
        ],
      ),
      MenuItem::new_submenu(
        "View",
        [
          custom_item("Dashboard")
            .accelerator("cmdOrControl+1")
            .into(),
          custom_item("Transactions")
            .accelerator("cmdOrControl+2")
            .into(),
          MenuItem::EnterFullScreen,
        ],
      ),
      MenuItem::new_submenu("Window", [MenuItem::Minimize, MenuItem::Zoom]),
      MenuItem::new_submenu("Help", [custom_item("Learn More").into()]),
    ]))
    .on_menu_event(|event| {
      let event_name = event.menu_item_id();
      let _ = event.window().emit("menu", event_name);
      match event_name {
        "Learn More" => {
          shell::open("https://github.com/probablykasper/kryp".to_string(), None).unwrap();
        }
        _ => {}
      }
    })
    .build(ctx)
    .expect("error while running tauri app");
  tauri_app.run(|app_handle, e| match e {
    tauri::Event::CloseRequested { label, api, .. } => {
      if label == "main" {
        let st: St<'_> = app_handle.state();
        let kryp = tauri::async_runtime::block_on(st.0.lock());
        if kryp.has_unsaved_changes() {
          api.prevent_close();
          let app_handle = app_handle.clone();
          let w = app_handle.get_window(&label).unwrap();
          let res = dialog_sync(
            w.clone(),
            "You have unsaved changes or newly fetched prices. Close without saving?",
            "",
          );
          if res == true {
            w.close().unwrap();
          }
        }
      }
    }
    _ => {}
  })
}

pub fn dialog_sync<S: AsRef<str>>(w: Window, title: S, msg: S) -> bool {
  let (sender, receiver) = std::sync::mpsc::channel();
  let title = title.as_ref().to_string();
  let msg = msg.as_ref().to_string();
  thread::spawn(move || {
    dialog::confirm(Some(&w), title, msg, move |res| {
      sender.send(res).unwrap();
    })
  });
  receiver.recv().unwrap_or(false)
}
