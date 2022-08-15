//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "package_dependencies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub package: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub dependency: String,
    pub relop: Option<String>,
    pub version: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub architecture: String,

    // PKGDEP, PKGRECOM, PKGBREAK, PKGCONFL, PKGREP,
    // PKGPROV, PKGSUG, BUILDDEP
    #[sea_orm(primary_key, auto_increment = false)]
    pub relationship: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::packages::Entity",
        from = "Column::Package",
        to = "super::packages::Column::Name",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Packages,
}

impl Related<super::packages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Packages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
