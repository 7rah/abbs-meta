//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "package_changes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub package: String,
    #[sea_orm(primary_key)]
    pub githash: String,

    pub version: String,
    pub branch: String,
    pub urgency: String,
    pub message: String,
    pub maintainer_name: String,
    pub maintainer_email: String,
    pub timestamp: i64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}