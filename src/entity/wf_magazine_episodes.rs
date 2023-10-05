//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wf_magazine_episodes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub id_magazine_revision: i32,
    pub id_episode: i32,
    pub image_top_first: Option<String>,
    pub image_top_second: Option<String>,
    pub image_bot_first: Option<String>,
    pub image_bot_second: Option<String>,
    pub custom_cta_text: Option<String>,
    pub custom_cta_url: Option<String>,
    pub poid: i32,
    pub new_ep: Option<i8>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}