// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod types;
use db::find_notes_by_tags;

use tauri::Manager;
use std::path::PathBuf;
pub struct DBPath(PathBuf);

fn main() {
    tauri::Builder::default()
        .setup(|app: &mut tauri::App| {
            let db_path: PathBuf = app.path_resolver().app_data_dir().unwrap().join("database.db");
            app.manage(DBPath(db_path));
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            find_notes_by_tags
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
