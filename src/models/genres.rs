use serde::{Serialize, Deserialize};
use sea_orm::FromQueryResult;

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct Genre {
    #[serde(rename = "id")]
    id_genre: i32,
    #[serde(rename = "genre")]
    genre: String,
    #[serde(rename = "class")]
    class: String
}
