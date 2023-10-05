/// All models from our series entity
use serde::{Serialize, Deserialize};
use sea_orm::FromQueryResult;

use super::authors::Author;
use super::genres::Genre;
use super::episodes::EpisodeList;

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct Serie {
    #[serde(rename = "id")]
    pub id_serie: i32,
    #[serde(rename = "title")]
    titre: String,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "likes")]
    aimes: i32,
    #[serde(rename = "readings")]
    lectures: i32,
    #[serde(rename = "coverIcon")]
    image_cover: String,
    #[serde(rename = "squareIcon")]
    image_carre: String,
    #[serde(rename = "verticalIcon")]
    image_vertical: String,
    #[serde(rename = "slug")]
    slug: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerieProcessed {
    #[serde(rename = "id")]
    pub id_serie: i32,
    #[serde(rename = "title")]
    titre: String,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "likes")]
    aimes: i32,
    #[serde(rename = "readings")]
    lectures: i32,
    #[serde(rename = "coverIcon")]
    image_cover: String,
    #[serde(rename = "squareIcon")]
    image_carre: String,
    #[serde(rename = "verticalIcon")]
    image_vertical: String,
    #[serde(rename = "slug")]
    slug: String,
    #[serde(rename = "hasNewEpisode")]
    new_ep: bool
}

///
/// Add a trait to the Serie struct to add data check after the initial query
/// 
pub trait SerieProcessData {
    fn process(&self, new_ep: bool) -> SerieProcessed;
}

///
/// Implement the trait to the struct and rebuild with a new struct
/// 
impl SerieProcessData for Serie {
    fn process(&self, new_ep: bool) -> SerieProcessed {
        SerieProcessed {
            id_serie:       self.id_serie.to_owned(),
            titre:          self.titre.to_owned(),
            description:    self.description.to_owned(),
            aimes:          self.aimes.to_owned(),
            lectures:       self.lectures.to_owned(),
            image_cover:    self.image_cover.to_owned(),
            image_carre:    self.image_carre.to_owned(),
            image_vertical: self.image_vertical.to_owned(),
            slug:           self.slug.to_owned(),
            new_ep:         new_ep
        }
    }
}

/// 
/// Combined structure of all needed
/// data for the single serie view
/// 
#[derive(Serialize, Deserialize, Debug)]
pub struct SerieJsonResponse {
    pub serie: SerieProcessed,
    pub authors: Vec<Author>,
    pub genres: Vec<Genre>,
    pub episodes: Vec<EpisodeList>
}

///
/// Combined structure of all needed
/// data for the list series view
/// 
#[derive(Serialize, Deserialize, Debug)]
pub struct SeriesListJsonResponse {
    pub serie: SerieProcessed,
    pub authors: Vec<Author>,
    pub genres: Vec<Genre>
}
