use super::{
    data::{CreateItemDTO, UpdateItemDTO},
    repository,
};
use crate::auth;
use entity::item;

#[tauri::command]
pub async fn create_item(
    item: CreateItemDTO,
    token: String,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<item::Model, String> {
    let Ok(_) = auth::jwt::validate_token(&token) else {
        return Err("Failed to authenticate token".to_string());
    };

    let conn = state.conn.lock().await;

    repository::create(&item, &conn).await
}

#[tauri::command]
pub async fn get_item_by_category(
    token: String,
    code: String,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<Vec<item::Model>, String> {
    let Ok(_) = auth::jwt::validate_token(&token) else {
        return Err("Failed to authenticate token".to_string());
    };

    let conn = state.conn.lock().await;

    repository::get_by_category(&code, &conn).await
}

#[tauri::command]
pub async fn get_item_by_description(
    token: String,
    description: String,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<Vec<item::Model>, String> {
    let Ok(_) = auth::jwt::validate_token(&token) else {
        return Err("Failed to authenticate token".to_string());
    };

    let conn = state.conn.lock().await;

    repository::get_by_description(&description, &conn).await
}

#[tauri::command]
pub async fn get_item_by_id(
    token: String,
    id: i32,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<item::Model, String> {
    let Ok(_) = auth::jwt::validate_token(&token) else {
        return Err("Failed to authenticate token".to_string());
    };

    let conn = state.conn.lock().await;

    repository::get_by_id(id, &conn).await
}

#[tauri::command]
pub async fn update_item(
    token: String,
    id: i32,
    item: UpdateItemDTO,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<item::Model, String> {
    let Ok(_) = auth::jwt::validate_token(&token) else {
        return Err("Failed to authenticate token".to_string());
    };

    let conn = state.conn.lock().await;

    repository::update(id, item, &conn).await
}
