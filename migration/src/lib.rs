pub use sea_orm_migration::prelude::*;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240823_131140_create_table_users::Migration),
            Box::new(m20240823_132522_create_table_post::Migration),
        ]
    }
}mod m20240823_131140_create_table_users;
mod m20240823_132522_create_table_post;
