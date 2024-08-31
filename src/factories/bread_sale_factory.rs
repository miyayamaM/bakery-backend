use crate::entities::bread_sale::ActiveModel as BreadSale;
use sea_orm::ActiveValue;

pub struct BreadSaleFactory;

impl BreadSaleFactory {
    pub fn create(bakery_id: &i32, bread_id: &i32) -> BreadSale {
        BreadSale {
            bakery_id: ActiveValue::Set(*bakery_id),
            bread_id: ActiveValue::Set(*bread_id),
            ..Default::default()
        }
    }
}
