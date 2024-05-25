use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "answers")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_serializing)]
    pub id: i32,
    #[serde(skip_serializing)]
    pub question_id: i32,
    pub text: String,
    pub is_correct: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Question,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Question => Entity::belongs_to(super::question::Entity)
                .from(Column::QuestionId)
                .to(super::question::Column::Id)
                .into(),
        }
    }
}

impl Related<super::question::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Question.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
