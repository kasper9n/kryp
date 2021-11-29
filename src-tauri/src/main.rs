#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::data::St;
use rust_decimal::{Decimal, RoundingStrategy};
use std::thread;
use tauri::api::{dialog, shell};
use tauri::{
  command, CustomMenuItem, Manager, Menu, MenuItem, Submenu, Window, WindowBuilder, WindowUrl,
};

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

fn custom_menu(name: &str) -> CustomMenuItem {
  let c = CustomMenuItem::new(name.to_string(), name);
  return c;
}

fn main() {
  let menu = Menu::new()
    .add_submenu(Submenu::new(
      // on macOS first menu is always app name
      "Kryp",
      Menu::new()
        .add_native_item(MenuItem::About("Kryp".to_string()))
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Services)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::HideOthers)
        .add_native_item(MenuItem::ShowAll)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit),
    ))
    .add_submenu(Submenu::new(
      "File",
      Menu::new()
        .add_item(custom_menu("New").accelerator("cmdOrControl+N"))
        .add_item(custom_menu("Open...").accelerator("cmdOrControl+O"))
        .add_native_item(MenuItem::Separator)
        .add_item(custom_menu("Save").accelerator("cmdOrControl+S"))
        .add_item(custom_menu("Save As...").accelerator("shift+cmdOrControl+S"))
        .add_item(custom_menu("Close").accelerator("cmdOrControl+W")),
    ))
    .add_submenu(Submenu::new("Edit", {
      let mut menu = Menu::new();
      menu = menu.add_native_item(MenuItem::Undo);
      menu = menu.add_native_item(MenuItem::Redo);
      menu = menu.add_native_item(MenuItem::Separator);
      menu = menu.add_native_item(MenuItem::Cut);
      menu = menu.add_native_item(MenuItem::Copy);
      menu = menu.add_native_item(MenuItem::Paste);
      #[cfg(not(target_os = "macos"))]
      {
        menu = menu.add_native_item(MenuItem::Separator);
      }
      menu = menu.add_native_item(MenuItem::SelectAll);
      menu
    }))
    .add_submenu(Submenu::new(
      "View",
      Menu::new()
        .add_item(custom_menu("Dashboard").accelerator("cmdOrControl+1"))
        .add_item(custom_menu("Transactions").accelerator("cmdOrControl+2"))
        .add_native_item(MenuItem::EnterFullScreen),
    ))
    .add_submenu(Submenu::new(
      "Window",
      Menu::new()
        .add_native_item(MenuItem::Minimize)
        .add_native_item(MenuItem::Zoom),
    ))
    .add_submenu(Submenu::new(
      "Help",
      Menu::new().add_item(custom_menu("Learn More")),
    ))
    .add_native_item(MenuItem::Copy);

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
      data::get_data,
      data::get_tax,
      data::get_transactions,
      data::add_transaction,
      data::get_holdings,
      data::get_prices,
    ])
    .menu(menu)
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
            "You have unsaved changes. Close without saving?",
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
