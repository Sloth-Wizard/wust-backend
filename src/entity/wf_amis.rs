//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wf_amis")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_ami: i32,
    pub date_crea: DateTime,
    pub date_modif: DateTime,
    #[sea_orm(unique)]
    pub email: String,
    pub mdp: String,
    pub mdp_token: Option<String>,
    pub mdp_date: Option<DateTime>,
    pub gg_id: Option<String>,
    pub fb_id: Option<String>,
    pub apple_id: Option<String>,
    pub date_connexion: Option<DateTime>,
    pub pseudo: String,
    pub pseudo_md5: String,
    pub pseudo_temp: Option<i8>,
    pub token: Option<String>,
    pub statut: i8,
    pub explicit: Option<i8>,
    pub explicit_code: Option<String>,
    pub explicit_lock: Option<i8>,
    #[sea_orm(column_type = "Text", nullable)]
    pub nom: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub prenom: Option<String>,
    pub civilite: i8,
    pub adresse: String,
    pub adresse2: String,
    pub adresse3: String,
    pub cpostal: String,
    pub ville: String,
    pub pays: String,
    pub naissance: DateTime,
    pub telfixe: String,
    pub telport: String,
    pub noabonne: String,
    pub notif_email: i8,
    pub notif_webpush: i8,
    pub lang: Option<String>,
    pub banni: Option<i8>,
    pub tech: Option<String>,
    pub special: Option<i8>,
    pub special_date: Option<DateTime>,
    pub jwt_secret: Option<String>,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_owned())", nullable)]
    pub firebase_token: Option<String>,
    pub valide: Option<i8>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}