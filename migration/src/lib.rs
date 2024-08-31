pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_bakery_table;
mod m20220101_000002_create_chef_table;
mod m20220101_000003_create_bread_table;
mod m20220101_000004_create_bread_sale_table;
mod m20220101_000005_add_columns_to_chef_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_bakery_table::Migration),
            Box::new(m20220101_000002_create_chef_table::Migration),
            Box::new(m20220101_000003_create_bread_table::Migration),
            Box::new(m20220101_000004_create_bread_sale_table::Migration),
            Box::new(m20220101_000005_add_columns_to_chef_table::Migration),
        ]
    }
}
