use factories::bakery_factory::BakeryFactory;
use futures::executor::block_on;
use sea_orm::{ColumnTrait, Database, DbErr, EntityTrait, LoaderTrait, QueryFilter};

mod entities;
mod factories;

use entities::{prelude::*, *};

const DATABASE_URL: &str = "postgres://postgres:postgres@127.0.0.1/bakery_backend";
async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let _bakery_id = BakeryFactory::create(&db).await;

    let bakeries: Vec<bakery::Model> = Bakery::find()
        .filter(bakery::Column::ProfitMargin.lte(0.2))
        .all(&db)
        .await
        .unwrap();

    println!("first: {:#?}", &bakeries);

    let breads: Vec<Vec<bread::Model>> = bakeries.load_many_to_many(Bread, BreadSale, &db).await?;

    println!("second: {:#?}", &breads);
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
