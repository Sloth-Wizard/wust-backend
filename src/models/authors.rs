use serde::{Serialize, Deserialize};
use sea_orm::FromQueryResult;

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct Author {
    #[serde(rename = "id")]
    id_auteur: i32,
    #[serde(rename = "name")]
    nom: String,
    #[serde(rename = "icon")]
    photo: String,
    #[serde(rename = "description")]
    description: String
}
