use sea_orm_migration::{prelude::*, schema::*};

use super::m20220101_000001_create_bakery_table::Bakery;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000002_create_chef_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chef::Table)
                    .if_not_exists()
                    .col(pk_auto(Chef::Id))
                    .col(string(Chef::Name))
                    .col(json_null(Chef::ContactDetails))
                    .col(integer(Chef::BakeryId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chef-bakery_id")
                            .from(Chef::Table, Chef::BakeryId)
                            .to(Bakery::Table, Bakery::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chef::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Chef {
    Table,
    Id,
    BakeryId,
    Name,
    ContactDetails,
}
