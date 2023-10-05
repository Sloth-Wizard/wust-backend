use sea_orm::entity::prelude::*; // To get access to Decimal & DateTime types

use serde::{Serialize, Deserialize};
use sea_orm::FromQueryResult;

///
/// Episode structure for a single ep view (for the reader)
/// 
/// TODO: add joined data related to the active session
/// 
/// 
/*
Example value
{
  "id": "episodeId",
  "serieId": "serieId",
  "seasonId": "seasonId",
  "title": "Croisée des chemins",
  "icon": "iconUrl.png",
  "startDate": "2019-05-10T06:00:00.000Z",
  "episodeNumber": "1",
  "seasonNumber": "1",
  "date": "2019-05-10T06:00:00.000Z",
  "likes": "42",
  "comments": "15",
  "isLiked": true,
  "isFollowed": true,
  "isNew": true,
  "banner": {
    "text": "Some txt on \n 2 lines",
    "label": "Donate here",
    "url": "Donate here"
  },
  "slug": "slug",
  "images": [
    {
      "page": "1",
      "url": "imageUrl.png",
      "width": "1080",
      "height": "1400"
    }
  ]
}

UPDATE `wf_saisons`
SET `wf_saisons`.date_crea = NULL
WHERE  CAST(`wf_saisons`.`date_crea` AS CHAR(20)) = "0000-00-00 00:00:00"
*/
/// 

///
/// Single episode struct for the reader
/// 
#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct Episode {
    #[serde(rename = "id")]
    id_webtoon: i32,
    #[serde(rename = "serieId")]
    serie_id: i32,
    #[serde(rename = "seasonId")]
    saison_id: i32,
    #[serde(rename = "title")]
    titre: String,
    #[serde(rename = "icon")]
    image: String,
    #[serde(rename = "startDate")]
    pub date_debut: DateTime,
    #[serde(rename = "episodeNumber")]
    episode: i32,
    #[serde(rename = "seasonNumber")]
    season: i32,
    #[serde(rename = "likes")]
    aimes: i32,
    #[serde(rename = "comments")]
    comments_count: i32,
    #[serde(rename = "slug")]
    slug: String,
    #[serde(rename = "images")]
    json: serde_json::Value
}

///
/// Episode data with an added spice to it
/// 
#[derive(Serialize, Deserialize, Debug)]
pub struct EnrichedEpisode {
    #[serde(rename = "id")]
    id_webtoon: i32,
    #[serde(rename = "serieId")]
    serie_id: i32,
    #[serde(rename = "seasonId")]
    saison_id: i32,
    #[serde(rename = "title")]
    titre: String,
    #[serde(rename = "icon")]
    image: String,
    #[serde(rename = "startDate")]
    date_debut: DateTime,
    #[serde(rename = "episodeNumber")]
    episode: i32,
    #[serde(rename = "seasonNumber")]
    season: i32,
    #[serde(rename = "likes")]
    aimes: i32,
    #[serde(rename = "comments")]
    comments_count: i32,
    #[serde(rename = "isLiked")]
    liked: bool,
    #[serde(rename = "isFollowed")]
    followed: bool,
    #[serde(rename = "isNew")]
    new_ep: bool,
    #[serde(rename = "slug")]
    slug: String,
    #[serde(rename = "images")]
    json: serde_json::Value
}

///
/// Add a trait to the Episode struct to add data check after the initial query
/// 
pub trait EpisodeProcessData {
    fn process(&self, liked: bool, followed: bool, new_ep: bool) -> EnrichedEpisode;
}

///
/// Implement the data from some functionality
/// 
impl EpisodeProcessData for Episode {
    fn process(&self, liked: bool, followed: bool, new_ep: bool) -> EnrichedEpisode {
        EnrichedEpisode {
            id_webtoon:     self.id_webtoon.to_owned(),
            serie_id:       self.serie_id.to_owned(),
            saison_id:      self.saison_id.to_owned(),
            titre:          self.titre.to_owned(),
            image:          self.image.to_owned(),
            date_debut:     self.date_debut.to_owned(),
            episode:        self.episode.to_owned(),
            season:         self.season.to_owned(),
            aimes:          self.aimes.to_owned(),
            comments_count: self.comments_count.to_owned(),
            liked:          liked,
            followed:       followed,
            new_ep:         new_ep,
            slug:           self.slug.to_owned(),
            json:           self.json.to_owned()
        }
    }
}

/// 
/// Episode structure for list view
/// 
#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct EpisodeList {
    #[serde(rename = "id")]
    id_webtoon: i32,
    #[serde(rename = "title")]
    titre: String,
    #[serde(rename = "startDate")]
    pub date_debut: DateTime,
    #[serde(rename = "episodeNumber")]
    episode: i32,
    #[serde(rename = "slug")]
    slug: String,
    #[serde(rename = "icon")]
    image: String,
    #[serde(rename = "price")]
    prix: Decimal,
    #[serde(rename = "ean")]
    ean: String
}

///
/// Episode struct to check only the date
/// 
#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct EpisodeNewStatus {
    #[serde(rename = "startDate")]
    pub date_debut: DateTime,
}
