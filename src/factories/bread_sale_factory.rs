use crate::{entities::bread_sale::ActiveModel, BreadSale};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

pub struct BreadSaleFactory;

impl BreadSaleFactory {
    pub async fn create(db: &DatabaseConnection, bakery_id: &i32, bread_id: &i32) {
        let bread_sale = ActiveModel {
            bakery_id: ActiveValue::Set(*bakery_id),
            bread_id: ActiveValue::Set(*bread_id),
            ..Default::default()
        };
        let _res = BreadSale::insert(bread_sale).exec(db).await.unwrap();
    }
}
