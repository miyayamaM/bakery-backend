use factories::bakery_factory::BakeryFactory;
use futures::executor::block_on;
use sea_orm::{ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
mod entities;
mod factories;

use entities::{prelude::*, *};

const DATABASE_URL: &str = "postgres://postgres:postgres@127.0.0.1/bakery_backend";
async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let _bakery_id = BakeryFactory::create(&db).await;

    get_bakeries_selling_melonpan(&db).await;
    get_reasonable_bakeries(&db).await;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}

async fn get_bakeries_selling_melonpan(db: &DatabaseConnection) {
    let bakeries: Vec<(bakery::Model, Vec<bread::Model>)> = Bakery::find()
        .find_with_related(Bread)
        .filter(bread::Column::Name.eq("メロンパン"))
        .all(db)
        .await
        .unwrap();

    println!("メロンパンを売っているパン屋: {:#?}", &bakeries);
}

async fn get_reasonable_bakeries(db: &DatabaseConnection) {
    // サブクエリを作成: 高価なパン (価格 > 1000) を持つパン屋の ID を探す
    let bakery_ids_selling_pricy_bread: Vec<i32> = Bread::find()
        .find_with_related(Bakery)
        .filter(bread::Column::Price.gt(1000))
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .flat_map(|(_bread, bakeries)| bakeries.into_iter().map(|bakery| bakery.id))
        .collect();

    // メインクエリ: 高価なパンが存在しないパン屋を取得
    let bakeries = Bakery::find()
        .filter(bakery::Column::Id.is_not_in(bakery_ids_selling_pricy_bread))
        .all(db)
        .await
        .unwrap();

    println!(
        "すべての商品が1000円未満のbakery（商品がないパン屋も含む）: {:#?}",
        &bakeries
    );
}
