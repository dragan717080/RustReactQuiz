use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::question::Entity")]
    Questions,
}

impl Related<super::answer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Questions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
