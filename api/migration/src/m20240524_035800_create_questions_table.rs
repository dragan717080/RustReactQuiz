use sea_orm_migration::prelude::*;

use super::m20240524_034739_create_categories_table::Category;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Question {
    #[sea_orm(iden = "questions")]
    Table,
    Id,
    CategoryName,
    Text,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Question::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Question::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Question::CategoryName).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_category_name")
                            .from(Question::Table, Question::CategoryName)
                            .to(Category::Table, Category::Name)
                    )
                    .col(ColumnDef::new(Question::Text).string().not_null().unique_key())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Question::Table).to_owned()).await
    }
}
