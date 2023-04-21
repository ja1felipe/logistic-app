use entity::category;

use crate::auth;

use super::data::CreateCategoryDTO;
use super::repository;

#[tauri::command]
pub async fn create_category(
    category: CreateCategoryDTO,
    token: String,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<category::Model, String> {
    let Ok(_) = auth::jwt::validate_token(&token) else {
        return Err("Error when validate token".to_string());
    };

    let conn = state.conn.lock().await;

    repository::create(&conn, &category).await
}

#[tauri::command]
pub async fn get_all_category(
    token: String,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<Vec<category::Model>, String> {
    let Ok(_) = auth::jwt::validate_token(&token) else {
        return Err("Error when validate token".to_string());
    };

    let conn = state.conn.lock().await;

    let categories = repository::get_all(&conn).await;

    match categories {
        Some(categories) => Ok(categories),
        None => Ok(Vec::new()),
    }
}

#[tauri::command]
pub async fn update_category(
    category: CreateCategoryDTO,
    id: i32,
    token: String,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<category::Model, String> {
    let Ok(_) = auth::jwt::validate_token(&token) else {
        return Err("Error when validate token".to_string());
    };

    let conn = state.conn.lock().await;

    repository::update(&conn, id, category).await
}

#[tauri::command]
pub async fn delete_category(
    id: i32,
    token: String,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<bool, String> {
    let Ok(_) = auth::jwt::validate_token(&token) else {
        return Err("Error when validate token".to_string());
    };

    let conn = state.conn.lock().await;

    let category = repository::delete(&conn, id).await;

    match category {
        Ok(category) => Ok(category),
        Err(_) => Err("Failed to delete category".to_string()),
    }
}
