use factories::bakery_factory::BakeryFactory;
use futures::executor::block_on;
use sea_orm::{Database, DbErr, EntityTrait};

mod entities;
mod factories;

use entities::{prelude::*, *};

const DATABASE_URL: &str = "postgres://postgres:postgres@127.0.0.1/bakery_backend";
async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let bakery_id = BakeryFactory::create(&db).await;

    let bakery = Bakery::find_by_id(bakery_id)
        .find_with_related(Bread)
        .all(&db)
        .await
        .unwrap();
    println!("{:#?}", bakery);

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
