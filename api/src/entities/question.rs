use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "questions")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_serializing)]
    pub id: i32,
    pub category_name: String,
    pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::answer::Entity")]
    Answers,
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryName",
        to = "super::category::Column::Name"
    )]
    Category,
}

impl Related<super::answer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Answers.def()
    }
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
