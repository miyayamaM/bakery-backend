use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000003_create_bread_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bread::Table)
                    .if_not_exists()
                    .col(pk_auto(Bread::Id))
                    .col(string(Bread::Name))
                    .col(integer(Bread::Price))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bread::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Bread {
    Table,
    Id,
    Name,
    Price,
}
