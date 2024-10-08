//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "bread")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub price: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::bread_sale::Entity")]
    BreadSale,
}

impl Related<super::bread_sale::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BreadSale.def()
    }
}

impl Related<super::bakery::Entity> for Entity {
    fn to() -> RelationDef {
        super::bread_sale::Relation::Bakery.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::bread_sale::Relation::Bread.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
