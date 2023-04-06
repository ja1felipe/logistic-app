use atelie_logistc::establish_db_connection;
use migration::DbErr;

mod seeds;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = establish_db_connection().await?;

    match seeds::user_seed::run(&db).await {
        Ok(_) => println!("User seeds run with success"),
        Err(_) => eprintln!("User seeds failed"),
    }

    Ok(())
}
