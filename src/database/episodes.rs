/// 
/// Episodes related queries
/// 
use sea_orm::{entity::*, error::*, query::*, DatabaseConnection};

use crate::entity::wf_webtoons as Episodes;
use crate::entity::wf_saisons as Seasons;
use crate::entity::wf_coms as Comments;

use crate::models::episodes as Models;

///
/// Get all episodes linked to a serie
/// 
/// ```
/// id -> id of a serie
/// ```
/// 
pub async fn get_episodes_by_serie_id(id: i32, db: &DatabaseConnection) -> Result<Vec<Models::EpisodeList>, DbErr> {
    let episodes: Vec<Models::EpisodeList> = Episodes::Entity::find()
        .filter(
            Condition::all()
                .add(Episodes::Column::SerieId.eq(id))
        )
        .order_by(Episodes::Column::Episode, Order::Asc)
        .into_model::<Models::EpisodeList>()
        .all(db)
        .await?;

    Ok(episodes)
}

///
/// Get only the date of the last episode of a serie
/// 
/// ```
/// id -> id of a serie
/// ```
/// 
/// TODO: merge what this function does into another by specifying a flag to the function params ?
/// 
pub async fn get_last_episode_date_by_serie_id(id: i32, db: &DatabaseConnection) -> Result<Option<Models::EpisodeNewStatus>, DbErr> {
    let episode: Option<Models::EpisodeNewStatus> = Episodes::Entity::find()
        .select_only()
        .column(Episodes::Column::DateDebut)
        .filter(
            Condition::all()
                .add(Episodes::Column::SerieId.eq(id))
        )
        .order_by(Episodes::Column::Episode, Order::Desc)
        .into_model::<Models::EpisodeNewStatus>()
        .one(db)
        .await?;

    Ok(episode)
}

///
/// Get a single episode
/// 
/// ```
/// id_serie -> id of a serie
/// id_episode -> id of an episode
/// ```
/// 
pub async fn get_episode_by_ids(id_serie: i32, id_episode: i32, db: &DatabaseConnection) -> Result<Option<Models::Episode>, DbErr> {
    let episode: Option<Models::Episode> = Episodes::Entity::find()
        .column_as(Seasons::Column::Numero, "season")
        .column_as(Comments::Column::IdCom.count(), "comments_count")
        .join_rev( // Get the season number of the ep
            JoinType::LeftJoin,
            Seasons::Entity::belongs_to(Episodes::Entity)
                .from(Seasons::Column::IdSaison)
                .to(Episodes::Column::SaisonId)
                .into()
        )
        .join_rev( // Count the comments on that ep
            JoinType::LeftJoin,
            Comments::Entity::belongs_to(Episodes::Entity)
                .from(Comments::Column::ObjetId)
                .to(Episodes::Column::IdWebtoon)
                .into()
        )
        .filter(
            Condition::all()
                .add(Episodes::Column::SerieId.eq(id_serie))
                .add(Episodes::Column::IdWebtoon.eq(id_episode))
                .add(Episodes::Column::Valide.eq(1))
        )
        .into_model::<Models::Episode>()
        .one(db)
        .await?;
    
    Ok(episode)
}

///
/// Get the first episode of a serie
/// 
/// ```
/// id_serie -> id of a serie
/// ```
/// TODO: Merge this function with `get_episode_by_ids()`
/// 
pub async fn get_episode_first_by_serie_id(id_serie: i32, db: &DatabaseConnection) -> Result<Option<Models::Episode>, DbErr> {
    let episode: Option<Models::Episode> = Episodes::Entity::find()
        .column_as(Seasons::Column::Numero, "season")
        .column_as(Comments::Column::IdCom.count(), "comments_count")
        .join_rev( // Get the season number of the ep
            JoinType::LeftJoin,
            Seasons::Entity::belongs_to(Episodes::Entity)
                .from(Seasons::Column::IdSaison)
                .to(Episodes::Column::SaisonId)
                .into()
        )
        .join_rev( // Count the comments on that ep
            JoinType::LeftJoin,
            Comments::Entity::belongs_to(Episodes::Entity)
                .from(Comments::Column::ObjetId)
                .to(Episodes::Column::IdWebtoon)
                .into()
        )
        .filter(
            Condition::all()
                .add(Episodes::Column::SerieId.eq(id_serie))
                .add(Episodes::Column::Episode.eq(1))
                .add(Episodes::Column::Valide.eq(1))
        )
        .into_model::<Models::Episode>()
        .one(db)
        .await?;
    
    Ok(episode)
}
