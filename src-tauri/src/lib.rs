#[cfg(target_os = "linux")]
use gtk::prelude::GtkWindowExt;

use tauri::Manager;

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
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
