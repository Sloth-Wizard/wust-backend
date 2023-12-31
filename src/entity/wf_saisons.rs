//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wf_saisons")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_saison: i32,
    pub date_crea: DateTime,
    pub date_modif: DateTime,
    pub serie_id: i32,
    pub numero: String,
    pub date_debut: Option<DateTime>,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub total_episodes: i32,
    pub complete: i8,
    pub day_planning: Option<String>,
    pub image_grande: String,
    pub image_vertical: String,
    pub image_cover: String,
    #[sea_orm(column_type = "Text")]
    pub fb_texte: String,
    pub fb_image: String,
    pub fb_image_w: i32,
    pub fb_image_h: i32,
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
