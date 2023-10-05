//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wf_signales")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_signale: i32,
    pub date_crea: DateTime,
    pub date_modif: DateTime,
    pub ami_id: i32,
    pub objet_type_id: i8,
    pub objet_id: i32,
    pub plus_moins: i8,
    pub valide: i8,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
