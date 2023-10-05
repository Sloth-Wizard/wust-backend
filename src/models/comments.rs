use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use sea_orm::FromQueryResult;

///
/// Single comment struct
/// 
#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct Comment {
    #[serde(rename = "id")]
    id_com: i32,
    #[serde(rename = "text")]
    commentaire: String,
    #[serde(rename = "pseudo")]
    pseudo: String,
    #[serde(rename = "userId")]
    ami_id: i32,
    #[serde(rename = "likes")]
    pouces: i32,
    #[serde(rename = "date")]
    date_crea: DateTime
}

///
/// Single comment with all answers linked to it
/// 
pub struct CommentEnriched {
    comment: Comment,
    answers: Vec<Comment>
}
