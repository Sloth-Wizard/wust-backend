//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wf_portal_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub identifiant: String,
    pub admin_id: i32,
    pub action: String,
    pub date_action: DateTime,
    pub ip: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
