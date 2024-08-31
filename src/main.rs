use futures::executor::block_on;
use sea_orm::{ActiveValue, Database, DbErr, EntityTrait};

mod entities;

use entities::{prelude::*, *};
use sea_orm_active_enums::EmploymentStatus;

const DATABASE_URL: &str = "postgres://postgres:postgres@127.0.0.1/bakery_backend";
async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };

    let res = Bakery::insert(happy_bakery).exec(&db).await.unwrap();

    let chef = chef::ActiveModel {
        name: ActiveValue::Set("Manu".into()),
        contact_details: ActiveValue::Set(Some("{}".into())),
        bakery_id: ActiveValue::Set(res.last_insert_id),
        employment_status: ActiveValue::Set(Some(EmploymentStatus::FullTime)),
        ..Default::default()
    };

    Chef::insert(chef).exec(&db).await.unwrap();

    for bread_name in ["フランスパン", "あんぱん", "クイニーアマン"] {
        let bread = bread::ActiveModel {
            name: ActiveValue::Set(bread_name.to_owned()),
            price: ActiveValue::Set(100.to_string()),
            ..Default::default()
        };
        let bread_res = Bread::insert(bread).exec(&db).await?;
        let breadsale = bread_sale::ActiveModel {
            bakery_id: ActiveValue::Set(res.last_insert_id),
            bread_id: ActiveValue::Set(bread_res.last_insert_id),
            ..Default::default()
        };
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
