//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wf_amis_aimes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_aime: i32,
    pub date_crea: DateTime,
    pub date_modif: DateTime,
    pub objet_type_id: i32,
    pub objet_id: i32,
    pub ami_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}