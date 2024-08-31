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

    let happy_bakery = BakeryFactory::create();

    let res = Bakery::insert(happy_bakery).exec(&db).await.unwrap();

    let chef = ChefFactory::create(&res.last_insert_id);

    Chef::insert(chef).exec(&db).await.unwrap();

    for _ in 0..=3 {
        let bread = BreadFactory::create();
        let bread_res = Bread::insert(bread).exec(&db).await?;
        let breadsale = BreadSaleFactory::create(&res.last_insert_id, &bread_res.last_insert_id);
        let _res = BreadSale::insert(breadsale).exec(&db).await?;
    }

    let bakery = Bakery::find_by_id(res.last_insert_id)
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
