use atelie_logistc::establish_db_connection;
use entity::user;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, Set};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = establish_db_connection().await?;

    let user = user::ActiveModel {
        email: Set(String::from("admin@admin.com")),
        name: Set(String::from("Admin")),
        password: Set(String::from("1234")),
        ..Default::default()
    };

    let user: user::Model = user.insert(&db).await?;

    println!("User created with ID: {}, EMAIL: {}", user.id, user.email);

    Ok(())
}
