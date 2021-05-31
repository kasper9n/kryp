#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::tax::{runtest, Tax};
use std::sync::{Arc, Mutex};
use tauri::{api, command, CustomMenuItem, Menu, MenuItem, State, WindowBuilder, WindowUrl};

mod prices;
mod tax;

#[macro_export]
macro_rules! throw {
  ($($arg:tt)*) => {{
    return Err(format!($($arg)*).to_owned())
  }};
}

#[derive(Debug, Default)]
struct Kryp {
  current: Option<Tax>,
}

#[derive(Default)]
struct Database(Arc<Mutex<Kryp>>);

#[command]
fn open(file_path: String, kryp: State<Database>) -> Result<(), String> {
  let mut kryp = kryp.0.lock().unwrap();
  if let None = kryp.current {
    kryp.current = Some(Tax::load(&file_path)?);
  }
  println!("fp {}", file_path);
  println!("tax {:?}", kryp.current);
  Ok(())
}

#[command]
fn error_popup(msg: String) {
  api::dialog::message("Error", msg);
}

#[command]
fn calculate() {
  runtest();
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
    .manage(Database(Default::default()))
    .invoke_handler(tauri::generate_handler![calculate, open, error_popup])
    .menu(menu)
    .on_menu_event(|event| match event.menu_item_id().as_str() {
      "learn-more" => {
        api::shell::open("https://github.com/probablykasper/kryp".to_string(), None).unwrap();
      }
      _ => {}
    })
    .run(ctx)
    .expect("error running application");
}
