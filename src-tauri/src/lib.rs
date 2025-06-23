use std::str::FromStr;

#[cfg(target_os = "linux")]
use gtk::prelude::GtkWindowExt;

use puzzle::scramble::BufferedScrambler;
use sqlx::{
    ConnectOptions,
    sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions},
};
use tauri::{Manager, path::BaseDirectory};
use tokio::sync::Mutex;

mod db;
mod entities;
mod handlers;
mod puzzle;

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
            let db_meta_path = app
                .path()
                .resolve("power-cube/db.meta", BaseDirectory::LocalData)
                .unwrap();
            let migrations_dir = app
                .path()
                .resolve("migrations", BaseDirectory::Resource)
                .unwrap();

            let app_state = AppState {
                scrambler: BufferedScrambler::new(java_dir),
                db_pool: SqlitePoolOptions::new()
                    .connect_lazy(&db_path.to_string_lossy())
                    .expect("unable to create connection pool"),
            };

            // passing the db_pool here is super clunky along with keeping it for the tauri state
            // to use, just going to create a new one in the closure
            tauri::async_runtime::spawn(async move {
                let conn = SqliteConnectOptions::from_str(&db_path.to_string_lossy())
                    .unwrap()
                    .connect()
                    .await
                    .expect("unable to create database connection for migrations");

                db::migrations::apply_migrations(conn, migrations_dir, db_meta_path)
                    .await
                    .expect("Failed to run database migrations");
            });

            app.manage(Mutex::new(app_state));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            handlers::get_scramble,
            handlers::scramble_cube,
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
