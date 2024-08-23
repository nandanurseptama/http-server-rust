use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveModel, Eq)]
#[sea_orm(table_name = "cake")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub text: String,
}