//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wf_bg_profile")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_bg: i32,
    pub date_crea: DateTime,
    pub date_debut: DateTime,
    pub date_modif: DateTime,
    pub image: String,
    pub couleur: String,
    pub titre: String,
    pub sous_titre: String,
    pub lang: String,
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
