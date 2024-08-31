use crate::{entities::bread::ActiveModel, Bread};
use fake::{Fake, Faker};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

pub struct BreadFactory;

impl BreadFactory {
    pub async fn create(db: &DatabaseConnection) -> i32 {
        let price = Faker.fake::<u16>();
        let bread = ActiveModel {
            name: ActiveValue::Set(Faker.fake::<String>()),
            price: ActiveValue::Set(price.into()),
            ..Default::default()
        };
        let res = Bread::insert(bread).exec(db).await.unwrap();
        res.last_insert_id
    }
}
