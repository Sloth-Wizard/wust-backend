//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wf_token_free_serie")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub id_serie: i32,
    pub date_crea: DateTime,
    pub date_modif: DateTime,
    pub date_debut: DateTime,
    pub date_fin: DateTime,
    pub valide: i8,
    pub token: String,
    pub pre_publish: i8,
    pub used: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}