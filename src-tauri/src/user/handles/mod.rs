use super::repository;
use entity::user;

#[tauri::command]
pub async fn get_all_users(
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<Vec<user::Model>, String> {
    let conn = state.conn.lock().await;
    let test = &*conn;
    let users = repository::get_all(test).await;

    match users {
        Some(usr) => Ok(usr),
        None => Err("hello".to_string()),
    }
}
