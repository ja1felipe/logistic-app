use entity::user;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

pub async fn run(db: &DatabaseConnection) -> Result<(), DbErr> {
    let user = user::ActiveModel {
        email: Set(String::from("admin@admin.com")),
        name: Set(String::from("Admin")),
        password: Set(String::from("1234")),
        ..Default::default()
    };

    let user: user::Model = user.insert(db).await?;

    println!("User created with ID: {}, EMAIL: {}", user.id, user.email);

    Ok(())
}
