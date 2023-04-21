// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tokio::sync::Mutex;

use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

mod auth;
mod category;
mod item;
mod user;

#[tokio::main]
async fn main() {
    let db: DatabaseConnection = atelie_logistc::establish_db_connection().await.unwrap();

    Migrator::up(&db, None).await.unwrap();

    let state = atelie_logistc::AppState {
        conn: Mutex::new(db),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            user::handles::get_all_users,
            user::handles::create_user,
            user::handles::login,
            category::handles::create_category,
            category::handles::get_all_category,
            category::handles::update_category,
            category::handles::delete_category,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
