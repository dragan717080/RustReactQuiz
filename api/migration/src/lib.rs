pub use sea_orm_migration::prelude::*;

mod m20240524_034739_create_categories_table;
mod m20240524_035800_create_questions_table;
mod m20240524_063817_create_answers_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240524_034739_create_categories_table::Migration),
            Box::new(m20240524_035800_create_questions_table::Migration),
            Box::new(m20240524_063817_create_answers_table::Migration),
        ]
    }
}
