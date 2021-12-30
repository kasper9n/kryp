#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::data::St;
use rust_decimal::{Decimal, RoundingStrategy};
use std::thread;
use tauri::api::{dialog, shell};
use tauri::{
  command, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu, Window, WindowBuilder,
  WindowUrl,
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
      MenuEntry::Submenu(Submenu::new(
        &ctx.package_info().name,
        Menu::with_items([
          MenuItem::About(ctx.package_info().name.clone()).into(),
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
          CustomMenuItem::new("Export...", "Export...")
            .accelerator("cmdOrControl+E")
            .into(),
          CustomMenuItem::new("Close", "Close")
            .accelerator("cmdOrControl+W")
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
  })
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
