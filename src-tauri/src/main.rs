#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use rust_decimal::{Decimal, RoundingStrategy};
use tauri::api::{dialog, shell};
use tauri::{command, CustomMenuItem, Menu, MenuItem, WindowBuilder, WindowUrl};

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
  let menu = vec![
    // on macOS first menu is always app name
    Menu::new(
      "Kryp",
      vec![
        MenuItem::About("Kryp".to_string()),
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
    Menu::new(
      "Edit",
      vec![
        MenuItem::Undo,
        MenuItem::Redo,
        MenuItem::Separator,
        MenuItem::Cut,
        MenuItem::Copy,
        MenuItem::Paste,
        MenuItem::Separator,
        MenuItem::SelectAll,
      ],
    ),
    Menu::new(
      "Help",
      vec![MenuItem::Custom(CustomMenuItem::new(
        "learn-more".into(),
        "Learn More",
      ))],
    ),
  ];

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
        .min_inner_size(300.0, 150.0)
        .fullscreen(false);
      return (win, webview);
    })
    .manage(data::Data(Default::default()))
    .invoke_handler(tauri::generate_handler![
      data::open,
      data::load_file,
      error_popup,
      data::save,
      data::get,
    ])
    .menu(menu)
    .on_menu_event(|event| match event.menu_item_id().as_str() {
      "learn-more" => {
        shell::open("https://github.com/probablykasper/kryp".to_string(), None).unwrap();
      }
      _ => {}
    })
    .run(ctx)
    .expect("error running application");
}
