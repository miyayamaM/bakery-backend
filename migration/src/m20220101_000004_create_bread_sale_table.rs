use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

use super::m20220101_000001_create_bakery_table::Bakery;
use super::m20220101_000003_create_bread_table::Bread;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000004_create_bread_sale_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BreadSale::Table)
                    .if_not_exists()
                    .col(pk_auto(BreadSale::Id))
                    .col(integer(BreadSale::BakeryId))
                    .col(integer(BreadSale::BreadId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-bread-sale-bakery_id")
                            .from(BreadSale::Table, BreadSale::BakeryId)
                            .to(Bakery::Table, Bakery::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-bread-sale-bread_id")
                            .from(BreadSale::Table, BreadSale::BreadId)
                            .to(Bread::Table, Bread::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BreadSale::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BreadSale {
    Table,
    Id,
    BakeryId,
    BreadId,
}
