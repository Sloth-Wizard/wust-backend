/// 
/// Genres related queries
/// 
use sea_orm::{entity::*, error::*, query::*, DatabaseConnection, JoinType};

use crate::entity::wf_genres as Genres;
use crate::entity::wf_series_genres as GenresSeries;

use crate::models::genres as Models;

/// 
/// Get all genres linked to a serie
/// 
/// ```
/// id -> id of a serie
/// ```
/// 
pub async fn get_genres_by_serie_id(id: i32, db: &DatabaseConnection) -> Result<Vec<Models::Genre>, DbErr> {
    let genres: Vec<Models::Genre> = Genres::Entity::find()
        .join_rev(
            JoinType::LeftJoin,
            GenresSeries::Entity::belongs_to(Genres::Entity)
                .from(GenresSeries::Column::GenreId)
                .to(Genres::Column::IdGenre)
                .into()
        )
        .filter(
            Condition::all()
                .add(GenresSeries::Column::SerieId.eq(id))
        )
        .into_model::<Models::Genre>()
        .all(db)
        .await?;

    Ok(genres)
}
