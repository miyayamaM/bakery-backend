use crate::entities::bakery::ActiveModel;
use crate::Bakery;
use fake::faker::address::en::*;
use fake::{Fake, Faker};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

use super::bread_factory::BreadFactory;
use super::bread_sale_factory::BreadSaleFactory;
use super::chef_factory::ChefFactory;

pub struct BakeryFactory;

impl BakeryFactory {
    pub async fn create(db: &DatabaseConnection) -> i32 {
        let name: String = CityName().fake();
        let bakery = ActiveModel {
            name: ActiveValue::Set(format!("{} bakery", name)),
            profit_margin: ActiveValue::Set(Faker.fake::<f64>()),
            ..Default::default()
        };

        let res = Bakery::insert(bakery).exec(db).await.unwrap();
        let bakery_id = res.last_insert_id;

        let _chef_id = ChefFactory::create(db, &bakery_id).await;

        for _ in 0..=3 {
            let bread_id = BreadFactory::create(db).await;
            let _breadsale = BreadSaleFactory::create(db, &bakery_id, &bread_id).await;
        }
        bakery_id
    }
}
