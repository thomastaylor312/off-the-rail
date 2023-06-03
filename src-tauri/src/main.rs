// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use diesel::{
    r2d2::{ConnectionManager, Pool},
    sqlite::SqliteConnection,
};

pub mod crud;
pub mod models;
pub mod schema;

pub struct DbConnection {
    conn: Pool<ConnectionManager<SqliteConnection>>,
}

fn main() {
    let data_dir = dirs::data_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("off-the-rail");
    if let Err(e) = std::fs::create_dir_all(&data_dir) {
        panic!(
            "Failed to create data directory {}: {e}",
            data_dir.display()
        );
    }
    let conn: ConnectionManager<SqliteConnection> =
        ConnectionManager::new(data_dir.join("database").to_str().unwrap().to_owned());
    tauri::Builder::default()
        .manage(DbConnection {
            conn: Pool::new(conn).expect("Unable to open connection to database"),
        })
        .invoke_handler(tauri::generate_handler![
            crud::classes::list_classes,
            crud::classes::update_class,
            crud::classes::create_class,
            crud::classes::delete_class,
            crud::classes::list_classes_for_division,
            crud::divisions::list_divisions,
            crud::divisions::update_division,
            crud::divisions::create_division,
            crud::divisions::delete_division,
            crud::shows::list_shows,
            crud::shows::update_show,
            crud::shows::create_show,
            crud::shows::delete_show,
            crud::shows::get_show,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
