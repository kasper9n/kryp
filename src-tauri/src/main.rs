#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod tax;

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
    .menu(menu)
    .on_menu_event(|event| match event.menu_item_id().as_str() {
      "learn-more" => {
        api::shell::open("https://github.com/probablykasper/kryp".to_string(), None).unwrap();
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
