use futures::executor::block_on;
use sea_orm::{ActiveValue, Database, DbErr, EntityTrait};

mod entities;

use entities::{prelude::*, *};

const DATABASE_URL: &str = "postgres://postgres:postgres@127.0.0.1/bakery_backend";
async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };

    let res = Bakery::insert(happy_bakery).exec(&db).await.unwrap();

    let john = chef::ActiveModel {
        name: ActiveValue::Set("John".to_owned()),
        bakery_id: ActiveValue::Set(res.last_insert_id),
        ..Default::default()
    };

    let _res = Chef::insert(john).exec(&db).await.unwrap();

    let bakery = Bakery::find_by_id(res.last_insert_id).one(&db).await;
    println!("{:#?}", bakery);

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
