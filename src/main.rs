use factories::{
    bakery_factory::BakeryFactory, bread_factory::BreadFactory,
    bread_sale_factory::BreadSaleFactory, chef_factory::ChefFactory,
};
use futures::executor::block_on;
use sea_orm::{Database, DbErr, EntityTrait};

mod entities;
mod factories;

use entities::{prelude::*, *};

const DATABASE_URL: &str = "postgres://postgres:postgres@127.0.0.1/bakery_backend";
async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let bakery_id = BakeryFactory::create(&db).await;
    let _chef_id = ChefFactory::create(&db, &bakery_id).await;

    for _ in 0..=3 {
        let bread_id = BreadFactory::create(&db).await;
        let _breadsale = BreadSaleFactory::create(&db, &bakery_id, &bread_id).await;
    }

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
