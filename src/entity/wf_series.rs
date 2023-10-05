//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wf_series")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_serie: i32,
    pub date_crea: DateTime,
    pub date_modif: DateTime,
    pub titre: String,
    pub titre_couleur: String,
    pub slug: String,
    pub accroche: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub explicit: i8,
    pub creator: Option<i8>,
    pub date_debut: DateTime,
    pub date_fin: DateTime,
    pub complete: i8,
    #[sea_orm(column_type = "Text")]
    pub technique: String,
    pub image_grande: String,
    pub image_cover: String,
    pub image_carre: String,
    pub image_vertical: String,
    pub r#ref: String,
    #[sea_orm(column_type = "Decimal(Some((4, 2)))")]
    pub prix: Decimal,
    #[sea_orm(column_type = "Decimal(Some((4, 2)))")]
    pub promo: Decimal,
    #[sea_orm(column_type = "Text")]
    pub reclame: String,
    pub aimes: i32,
    pub lectures: i32,
    pub debloques: i32,
    #[sea_orm(column_type = "Text")]
    pub fb_texte: String,
    pub fb_image: String,
    pub fb_image_w: i32,
    pub fb_image_h: i32,
    pub lang: String,
    pub w_onboarding: Option<i32>,
    pub valide_site: i8,
    pub valide: i8,
    pub valide_abospirou: i8,
    pub date_start_spi: DateTime,
    pub date_end_spi: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
