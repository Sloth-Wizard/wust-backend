/// 
/// Authors related queries
/// 
use sea_orm::{entity::*, error::*, query::*, DatabaseConnection, JoinType};

use crate::entity::wf_auteurs as Authors;
use crate::entity::wf_series_auteurs as AuthorsSeries;

use crate::models::authors as Models;

/// 
/// Get all authors linked to a serie
/// 
/// ```
/// id -> id of a serie
/// ```
/// 
pub async fn get_authors_by_serie_id(id: i32, db: &DatabaseConnection) -> Result<Vec<Models::Author>, DbErr> {
    let authors: Vec<Models::Author> = Authors::Entity::find()
        .join_rev(
            JoinType::LeftJoin,
            AuthorsSeries::Entity::belongs_to(Authors::Entity)
                .from(AuthorsSeries::Column::AuteurId)
                .to(Authors::Column::IdAuteur)
                .into()
        )
        .filter(
            Condition::all()
                .add(AuthorsSeries::Column::SerieId.eq(id))
        )
        .into_model::<Models::Author>()
        .all(db)
        .await?;

    Ok(authors)
}
