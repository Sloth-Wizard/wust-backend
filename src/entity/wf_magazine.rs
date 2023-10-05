//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wf_magazine")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub titre: String,
    pub slug: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub couleur: String,
    pub image_magazine: String,
    pub image_carousel_magazine: String,
    pub custom_cta_suivre: Option<String>,
    pub custom_cta_suivre_mag: Option<String>,
    pub date_crea: DateTime,
    pub date_modif: DateTime,
    pub date_debut: Option<DateTime>,
    pub date_fin: Option<DateTime>,
    pub valide: i8,
    pub lang: String,
    pub aimes: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}