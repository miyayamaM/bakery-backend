use crate::entities::chef::ActiveModel;
use crate::sea_orm_active_enums::EmploymentStatus;
use crate::Chef;
use fake::faker::name::en::Name;
use fake::Fake;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

pub struct ChefFactory;

impl ChefFactory {
    pub async fn create(db: &DatabaseConnection, bakery_id: &i32) -> i32 {
        let chef = ActiveModel {
            name: ActiveValue::Set(Name().fake()),
            contact_details: ActiveValue::Set(Some("{}".into())),
            bakery_id: ActiveValue::Set(*bakery_id),
            employment_status: ActiveValue::Set(Some(EmploymentStatus::FullTime)),
            ..Default::default()
        };

        let res = Chef::insert(chef).exec(db).await.unwrap();
        res.last_insert_id
    }
}
