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

    for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_owned()),
            bakery_id: ActiveValue::Set(res.last_insert_id),
            contact_details: ActiveValue::Set("{}".into()),
            ..Default::default()
        };
        Chef::insert(chef).exec(&db).await?;
    }

    let bakery = Bakery::find_by_id(res.last_insert_id)
        .find_with_related(Chef)
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
