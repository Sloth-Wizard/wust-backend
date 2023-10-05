/// 
/// Comments related queries
/// 
/// Usefull information =>
///     Left side is the id
///     Right side is the type it is linked to
/// 
///     ``` 
///     objet_type_id {
///         2 -> serie
///         3 -> webtoon
///         4 -> com
///     }
///     ```
/// 
use sea_orm::{entity::*, error::*, query::*, DatabaseConnection};

use crate::entity::wf_coms as Comments;

use crate::models::comments as Models;

///
/// Get comments related to an episode
/// 
/// ```
/// id -> id of the entity the comment is linked to
/// id_type -> id of the type the comments should be linked to
/// ```
/// 
pub async fn get_comments(id: i32, id_type: i32, db: &DatabaseConnection) -> Result<Vec<Models::Comment>, DbErr> {
    let comments: Vec<Models::Comment> = Comments::Entity::find()
        .filter(
            Condition::all()
                .add(Comments::Column::ObjetTypeId.eq(id_type))
                .add(Comments::Column::ObjetId.eq(id))
                .add(Comments::Column::Valide.eq(1))
        )
        .into_model::<Models::Comment>()
        .all(db)
        .await?;

    Ok(comments)
}
