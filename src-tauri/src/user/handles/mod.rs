use super::data;
use super::repository;
use crate::auth;
use crate::auth::jwt::generate_token;
use entity::user;

#[tauri::command]
pub async fn get_all_users(
    token: String,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<Vec<user::Model>, String> {
    let Ok(_user) = auth::jwt::validate_token(&token) else {
        return Err("Error when validate token".to_string());
    };

    let conn = state.conn.lock().await;
    let users = repository::get_all(&conn).await;

    match users {
        Some(usr) => Ok(usr),
        None => Ok(Vec::new()),
    }
}

#[tauri::command]
pub async fn create_user(
    new_user: data::CreateUserInputDTO,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<user::Model, String> {
    println!("{:?}", new_user);
    let conn = state.conn.lock().await;

    repository::create(&conn, &new_user).await
}

#[tauri::command]
pub async fn login(
    user: data::LoginUserInputDTO,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<String, String> {
    println!("{:?}", user);
    let conn = state.conn.lock().await;
    let user = repository::login(&conn, &user).await;

    match user {
        Ok(user) => {
            let token = generate_token(user).unwrap();
            Ok(token)
        }
        Err(err) => Err(err),
    }
}
