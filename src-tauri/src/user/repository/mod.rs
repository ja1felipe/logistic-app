use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use entity::user::{self, Entity as User};
use migration::DbErr;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::json;

use super::data::{CreateUserInputDTO, LoginUserInputDTO};

pub async fn get_all(db: &DatabaseConnection) -> Option<Vec<user::Model>> {
    let users: Result<Vec<user::Model>, DbErr> = User::find().all(db).await;

    match users {
        Ok(us) => Some(us),
        Err(_) => None,
    }
}

pub async fn create(
    db: &DatabaseConnection,
    new_user: &CreateUserInputDTO,
) -> Result<user::Model, String> {
    let password = new_user.password.clone();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hard = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let Ok(mut user) = user::ActiveModel::from_json(json!(new_user)) else {
        return Err("Failed creating a user".to_string());
    };

    user.set(user::Column::Password, password_hard.into());

    match user.insert(db).await {
        Ok(user) => Ok(user),
        Err(_) => Err("Failed creating a user".to_string()),
    }
}

pub async fn login(
    db: &DatabaseConnection,
    user: &LoginUserInputDTO,
) -> Result<user::Model, String> {
    let Ok(db_user) = User::find()
        .filter(user::Column::Email.eq(&user.email))
        .one(db)
        .await else {
            return Err("Failed to find user.".to_string());
        };

    match db_user {
        Some(db_user) => {
            let send_pw = user.password.clone();
            let db_pw = db_user.password.clone();
            let parsed_hash = PasswordHash::new(&db_pw).unwrap();
            if Argon2::default()
                .verify_password(send_pw.as_bytes(), &parsed_hash)
                .is_ok()
            {
                Ok(db_user)
            } else {
                Err("Incorrect password.".to_string())
            }
        }
        None => Err("No user found with this email".to_string()),
    }
}
