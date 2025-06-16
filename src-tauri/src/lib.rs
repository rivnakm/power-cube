use std::sync::Mutex;

#[cfg(target_os = "linux")]
use gtk::prelude::GtkWindowExt;

use scramble::BufferedScrambler;
use tauri::{path::BaseDirectory, Manager};

mod db;
mod handlers;
mod models;
mod scramble;

struct AppState {
    scrambler: BufferedScrambler,
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Workaround for this issue
            // https://github.com/tauri-apps/tauri/issues/12955
            // https://github.com/tauri-apps/tao/issues/1046
            #[cfg(target_os = "linux")]
            {
                let window = app
                    .get_webview_window("main")
                    .ok_or("'main' WebviewWindow not found")?;
                let gtk_window = window.gtk_window()?;
                gtk_window.set_titlebar(Option::<&gtk::Widget>::None);
            }

            let java_dir = app.path().resolve("j4rs", BaseDirectory::Resource).unwrap();
            app.manage(Mutex::new(AppState {
                scrambler: BufferedScrambler::new(java_dir),
            }));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            handlers::get_scramble,
            handlers::record_solve
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
