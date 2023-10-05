//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wf_carrousels")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_carrousel: i32,
    pub date_crea: DateTime,
    pub date_modif: DateTime,
    pub titre: String,
    pub logo_titre: Option<String>,
    pub titre_style: String,
    pub slug: String,
    pub rubrique: String,
    pub mots: String,
    pub date_debut: DateTime,
    pub date_fin: DateTime,
    #[sea_orm(column_type = "Text")]
    pub intro: String,
    #[sea_orm(column_type = "Text")]
    pub contenu: String,
    pub text_color: Option<String>,
    pub lien: String,
    pub lien_ext: String,
    pub image: String,
    pub image_app: String,
    pub afficher_app: i8,
    pub serie_id: i32,
    pub background: String,
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