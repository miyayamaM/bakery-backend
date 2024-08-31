use crate::entities::chef::ActiveModel as Chef;
use crate::sea_orm_active_enums::EmploymentStatus;
use fake::faker::name::en::Name;
use fake::Fake;
use sea_orm::ActiveValue;

pub struct ChefFactory;

impl ChefFactory {
    pub fn create(bakery_id: &i32) -> Chef {
        Chef {
            name: ActiveValue::Set(Name().fake()),
            contact_details: ActiveValue::Set(Some("{}".into())),
            bakery_id: ActiveValue::Set(*bakery_id),
            employment_status: ActiveValue::Set(Some(EmploymentStatus::FullTime)),
            ..Default::default()
        }
    }
}
