use crate::entities::bread::ActiveModel as Bread;
use fake::{Fake, Faker};
use sea_orm::ActiveValue;

pub struct BreadFactory;

impl BreadFactory {
    pub fn create() -> Bread {
        let price = Faker.fake::<u16>();
        Bread {
            name: ActiveValue::Set(Faker.fake::<String>()),
            price: ActiveValue::Set(price.into()),
            ..Default::default()
        }
    }
}
