use crate::extension::postgres::Type;
use sea_orm::{DeriveActiveEnum, EnumIter, Iterable};
use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000005_add_columns_to_chef_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("employment_status"))
                    .values(vec![Alias::new("full_time"), Alias::new("part_time")])
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Chef::Table)
                    .add_column(
                        ColumnDef::new(Chef::EmploymentStatus)
                            .enumeration(Alias::new("employment_status"), EmploymentStatus::iter()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Chef::Table)
                    .drop_column(Alias::new("employment_status"))
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(Alias::new("employment_status"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Chef {
    Table,
    EmploymentStatus,
}

#[derive(Iden, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tea_type")]
enum EmploymentStatus {
    #[sea_orm(string_value = "FullTime")]
    FullTime,
    #[sea_orm(string_value = "PartTime")]
    PartTime,
    #[sea_orm(string_value = "Internship")]
    Internship,
}
