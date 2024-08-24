use futures::executor::block_on;
use sea_orm::{Database, DbErr};

const DATABASE_URL: &str = "postgres://postgres:postgres@127.0.0.1/bakery_backend";
async fn run() -> Result<(), DbErr> {
    let _db = Database::connect(DATABASE_URL).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
