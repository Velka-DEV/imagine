pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_file_table;
mod m20230813_121555_create_image_table;
mod m20230813_233040_create_image_variant_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_file_table::Migration),
            Box::new(m20230813_121555_create_image_table::Migration),
            Box::new(m20230813_233040_create_image_variant_table::Migration),
        ]
    }
}
