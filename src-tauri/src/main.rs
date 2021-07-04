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

fn custom_menu(name: &str) -> CustomMenuItem<String> {
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
        .add_item(custom_menu("Save").disabled().accelerator("cmdOrControl+S"))
        .add_item(
          custom_menu("Save As...")
            .disabled()
            .accelerator("shift+cmdOrControl+S"),
        )
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
        .add_item(custom_menu("Transactions").accelerator("cmdOrControl+2")),
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
