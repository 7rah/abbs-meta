//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "package_duplicate")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub package: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tree: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub category: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub section: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub directory: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}