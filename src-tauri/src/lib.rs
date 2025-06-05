#[cfg(target_os = "linux")]
use gtk::prelude::GtkWindowExt;

use tauri::{path::BaseDirectory, Manager};
use tnoodle_rs::{
    jvm,
    puzzle::{Puzzle, PuzzleType},
};

#[tauri::command]
fn get_scramble(app_handle: tauri::AppHandle) -> String {
    let app_dir = app_handle
        .path()
        .resolve("j4rs", BaseDirectory::Resource)
        .unwrap();

    // TODO: fallback to `None` when not bundled
    // TODO: set up ci to download artifacts from maven before bundle step. running cargo test
    // --release might be sufficient
    let jvm = jvm::get_jvm(Some(&app_dir)).unwrap();
    let puzzle = Puzzle::new(jvm, PuzzleType::Three).unwrap();
    puzzle.generate_wca_scramble().unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
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

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_scramble])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
