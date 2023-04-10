use super::data;
use super::repository;
use entity::user;
#[tauri::command]
pub async fn get_all_users(
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<Vec<user::Model>, String> {
    let conn = state.conn.lock().await;
    let users = repository::get_all(&conn).await;

    match users {
        Some(usr) => Ok(usr),
        None => Err("hello".to_string()),
    }
}

#[tauri::command]
pub async fn create_user(
    new_user: data::CreateUserInputDTO,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<user::Model, String> {
    println!("{:?}", new_user);
    let conn = state.conn.lock().await;
    let user = repository::create(&conn, &new_user).await;

    match user {
        Ok(user) => Ok(user),
        Err(_) => Err("Failed to create new user.".to_string()),
    }
}

#[tauri::command]
pub async fn login(
    user: data::LoginUserInputDTO,
    state: tauri::State<'_, atelie_logistc::AppState>,
) -> Result<user::Model, String> {
    println!("{:?}", user);
    let conn = state.conn.lock().await;
    let user = repository::login(&conn, &user).await;

    match user {
        Ok(user) => Ok(user),
        Err(err) => Err(err),
    }
}
