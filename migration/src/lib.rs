pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261107_013657_create_content::Migration),
            Box::new(m20261107_013703_create_content_part::Migration),
        ]
    }
}
mod m20261107_013657_create_content;
mod m20261107_013703_create_content_part;
