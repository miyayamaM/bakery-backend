use crate::entities::bakery::ActiveModel as Bakery;
use fake::faker::address::en::*;
use fake::{Fake, Faker};
use sea_orm::ActiveValue;

pub struct BakeryFactory;

impl BakeryFactory {
    pub fn create() -> Bakery {
        let name: String = CityName().fake();
        Bakery {
            name: ActiveValue::Set(format!("{} bakery", name)),
            profit_margin: ActiveValue::Set(Faker.fake::<f64>()),
            ..Default::default()
        }
    }
}
