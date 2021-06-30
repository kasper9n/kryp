#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use rust_decimal::{Decimal, RoundingStrategy};
use tauri::api::{dialog, shell};
use tauri::{command, CustomMenuItem, Menu, MenuItem, Submenu, WindowBuilder, WindowUrl};

mod data;
mod prices;
mod tax;

#[command]
fn error_popup(msg: String) {
  println!("Error popup: {}", msg);
  dialog::message("Error", msg);
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
      "Edit",
      Menu::new()
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::SelectAll),
    ))
    .add_submenu(Submenu::new(
      "View",
      Menu::new()
        .add_item(
          CustomMenuItem::new("Dashboard".into(), "Dashboard").accelerator("cmdOrControl+1"),
        )
        .add_item(
          CustomMenuItem::new("Transactions".into(), "Transactions").accelerator("cmdOrControl+2"),
        ),
    ))
    .add_submenu(Submenu::new(
      "Help",
      Menu::new().add_item(CustomMenuItem::new("Learn More".into(), "Learn More")),
    ))
    .add_native_item(MenuItem::Copy);

  let ctx = tauri::generate_context!();
  tauri::Builder::default()
    .create_window("main".into(), WindowUrl::default(), |win, webview| {
      let win = win
        .title("Kryp")
        .resizable(true)
        .transparent(false)
        .decorations(true)
        .always_on_top(false)
        .inner_size(1000.0, 800.0)
        .min_inner_size(300.0, 200.0)
        .fullscreen(false);
      return (win, webview);
    })
    .manage(data::Data(Default::default()))
    .invoke_handler(tauri::generate_handler![
      data::open,
      data::load_file,
      error_popup,
      data::save,
      data::get_data,
      data::get_tax,
      data::get_transactions,
      data::add_transaction,
      data::get_balances_by_asset,
    ])
    .menu(menu)
    .on_menu_event(|event| {
      let event_name = event.menu_item_id().as_str();
      let _ = event.window().emit("menu", event_name);
      match event_name {
        "Learn More" => {
          shell::open("https://github.com/probablykasper/kryp".to_string(), None).unwrap();
        }
        _ => {}
      }
    })
    .run(ctx)
    .expect("error running application");
}
