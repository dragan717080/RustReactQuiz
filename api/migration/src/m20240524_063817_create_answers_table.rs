use sea_orm_migration::prelude::*;

use super::m20240524_035800_create_questions_table::Question;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Answer {
    #[sea_orm(iden = "answers")]
    Table,
    Id,
    QuestionId,
    Text,
    IsCorrect,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Answer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Answer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Answer::Text).string().not_null())
                    .col(ColumnDef::new(Answer::QuestionId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_question_id")
                            .from(Answer::Table, Answer::QuestionId)
                            .to(Question::Table, Question::Id)
                    )
                    .col(ColumnDef::new(Answer::IsCorrect).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Answer::Table).to_owned()).await
    }
}
