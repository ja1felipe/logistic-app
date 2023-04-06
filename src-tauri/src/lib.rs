use dotenv::dotenv;
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbConn};
use tokio::sync::Mutex;

pub async fn establish_db_connection() -> Result<DbConn, DbErr> {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let db = Database::connect(&url)
        .await
        .expect("Failed to setup database");

    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    Ok(db)
}

pub struct AppState {
    pub conn: Mutex<DatabaseConnection>,
}
