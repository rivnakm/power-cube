#[cfg(target_os = "linux")]
use gtk::prelude::GtkWindowExt;

use scramble::BufferedScrambler;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use tauri::{Manager, path::BaseDirectory};
use tokio::sync::Mutex;

mod db;
mod handlers;
mod models;
mod scramble;

struct AppState {
    scrambler: BufferedScrambler,
    db_pool: SqlitePool,
}

pub async fn run() {
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
            let db_path = app
                .path()
                .resolve("power-cube/db.sqlite", BaseDirectory::LocalData)
                .unwrap();

            eprintln!("{:?}", db_path);

            app.manage(Mutex::new(AppState {
                scrambler: BufferedScrambler::new(java_dir),
                db_pool: SqlitePoolOptions::new()
                    .connect_lazy(&db_path.to_string_lossy())
                    .expect("unable to create connection pool"),
            }));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            handlers::get_scramble,
            handlers::record_solve,
            handlers::get_all_solves,
            handlers::delete_solve,
            handlers::get_avg_of_n,
            handlers::get_best_time,
            handlers::get_best_avg_of_n
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
