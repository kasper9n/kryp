#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
#![allow(warnings)]

use data::{Data, Kryp};
use localzone;
use rust_decimal::{Decimal, RoundingStrategy};
use specta_typescript::Typescript;
use tauri::async_runtime::Mutex;
use tauri::{
	command, AppHandle, Manager, RunEvent, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent,
};
use tauri_plugin_dialog::{DialogExt, FilePath, MessageDialogButtons, MessageDialogKind};
use tauri_specta::{collect_commands, Builder};

mod calc;
mod data;
mod export;
mod fetch;
mod fetch_current;
mod get_transactions;
mod holdings;
mod import;
mod prices;
mod reports;
mod tax;
mod transaction;

fn save_csv_tsv(_win: &Window, file_name: &str) -> Option<FilePath> {
	let d = _win
		.dialog()
		.file()
		.add_filter("Table", &["csv", "tsv"])
		.set_file_name(file_name)
		.set_parent(&_win)
		.blocking_save_file();
	return d;
}

#[command]
#[specta::specta]
fn error_popup(msg: String, win: Window) {
	println!("Error: {}", msg);
	win.app_handle()
		.dialog()
		.message(msg)
		.kind(MessageDialogKind::Error);
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

#[command]
#[specta::specta]
fn get_system_timezone() -> Option<String> {
	localzone::get_local_zone()
}

fn main() {
	let mut builder = Builder::<tauri::Wry>::new()
		// Then register them (separated by a comma)
		.commands(collect_commands![
			error_popup,
			get_system_timezone,
			data::new_file,
			data::open,
			data::save,
			data::close,
			data::is_open,
			data::get_tax,
			data::get_tax_settings,
			data::add_transaction,
			holdings::get_holdings,
			holdings::get_holdings_valued,
			holdings::get_holdings_by_wallet,
			data::list_assets,
			data::get_prices,
			reports::get_deposit_withdrawal_tags,
			reports::get_report,
			reports::download_report,
			import::scan_import_file,
			import::get_import_data,
			import::update_import_transactions,
			import::continue_import,
			import::cancel_import,
			export::export,
			get_transactions::get_transactions,
		]);

	#[cfg(debug_assertions)] // <- Only export on non-release builds
	builder
		.export(
			Typescript::default().bigint(specta_typescript::BigIntExportBehavior::Number),
			"../bindings.ts",
		)
		.expect("Failed to export typescript bindings");

	let ctx = tauri::generate_context!();
	let app = tauri::Builder::default()
		.plugin(tauri_plugin_window_state::Builder::new().build())
		.plugin(tauri_plugin_opener::init())
		.plugin(tauri_plugin_dialog::init())
		.manage(import::ImportData::default())
		.invoke_handler(builder.invoke_handler())
		.setup(move |app| {
			// This is also required if you want to use events
			builder.mount_events(app);

			app.manage(data::Data(Mutex::new(Kryp::new(app.app_handle().clone()))));
			let _ = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
				.title("Kryp")
				.resizable(true)
				.decorations(true)
				.always_on_top(false)
				.inner_size(1050.0, 800.0)
				.min_inner_size(300.0, 200.0)
				.fullscreen(false)
				.visible(false) // tauri_plugin_window_state will reveal the window
				.build()?;

			Ok(())
		})
		.build(ctx)
		.expect("error while running tauri app");

	app.run(app_run)
}

fn app_run(app_handle: &AppHandle, run_event: RunEvent) {
	match run_event {
		tauri::RunEvent::WindowEvent { label, event, .. } => {
			handle_window_event(app_handle, label, event)
		}
		_ => {}
	}
}

fn handle_window_event(app: &AppHandle, label: String, event: WindowEvent) {
	match event {
		WindowEvent::CloseRequested { api, .. } => {
			if label == "main" {
				let st = app.state::<Data>();
				let kryp = tauri::async_runtime::block_on(st.0.lock());
				if kryp.has_unsaved_changes() {
					api.prevent_close();
					let app_handle = app.clone();
					let w = app_handle.get_webview_window(&label).unwrap();
					let title =
						"You have unsaved changes or newly fetched prices. Close without saving?";
					app.dialog()
						.message(title)
						.parent(&w)
						.buttons(MessageDialogButtons::OkCancel)
						.show(move |response| {
							if response == true {
								w.destroy().unwrap();
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
	w.dialog()
		.message(msg)
		.parent(&w)
		.title(title)
		.buttons(MessageDialogButtons::OkCancel)
		.show(|response| {
			sender.send(response).unwrap();
		});
	receiver.await.unwrap_or(false)
}
