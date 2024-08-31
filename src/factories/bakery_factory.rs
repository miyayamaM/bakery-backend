use crate::entities::bakery::ActiveModel;
use crate::Bakery;
use fake::faker::address::en::*;
use fake::{Fake, Faker};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

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
        res.last_insert_id
    }
}
