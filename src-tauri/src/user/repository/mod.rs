use entity::user::{self, Entity as User};
use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn get_all(db: &DatabaseConnection) -> Option<Vec<user::Model>> {
    let users: Result<Vec<user::Model>, DbErr> = User::find().all(db).await;

    match users {
        Ok(us) => Some(us),
        Err(_) => None,
    }
}
