/// 
/// Series related queries
/// 
/// Option type is for single entity
/// Vec type is for list of entities
/// 
/// If you'd want to join another table
/// that should return one value and not
/// multiple, we can do it like this
///     ```rust
///     .join_rev(
///         JoinType::LeftJoin,
///         SeriesAuthorsEntity::belongs_to(SeriesEntity)
///             .from(SeriesAuthorsColumn::SerieId)
///             .to(SeriesColumn::IdSerie)
///             .into()
///         )
///      )
///     ```
/// 
/// Example of raw SQL query func
///     ```rust
///     pub async fn _get_serie_by_id_raw(id: i32) -> Result<Option<Serie>, DbErr> {
///         let db = Database::connect("mysql://root@127.0.0.1:3306/wf")
///            .await
///            .unwrap();
///
///         let query: Option<Serie> = Serie::find_by_statement(
///             Statement::from_sql_and_values(
///                 DbBackend::MySql, 
///                 &format!("
///                     SELECT ws.*, wsa.*
///                     FROM wf_series AS ws
///                     LEFT JOIN wf_series_auteurs AS wsa ON ws.id_serie = wsa.serie_id
///                     WHERE ws.id_serie = {}
///                     AND ws.valide = 1",
///                        id
///                 ),
///                 vec![],
///             ))
///             .one(&db)
///             .await?;
///
///         Ok(query)
///     }
///     ```
/// 
/// TODO: add dates verification to our queries to avoid returning not released series yet
/// 
/// ```
/// use chrono::Utc;
/// let _now = Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S");
/// ```
/// 
use sea_orm::{entity::*, error::*, query::*, DatabaseConnection, JoinType, Condition};

use crate::entity::wf_series as Series;
use crate::entity::wf_auteurs as Authors;
use crate::entity::wf_series_auteurs as AuthorsSeries;
use crate::entity::wf_genres as Genres;
use crate::entity::wf_series_genres as GenresSeries;

use crate::models::series as Models;

///
/// Get a list of 10 series
/// 
/// ```
/// limit -> limits the number of series returned by our Vec
/// explicit -> series with possible nudity
/// lang -> language of the device
/// ```
///
pub async fn get_series(limit: u64, explicit: bool, lang: String, db: &DatabaseConnection) -> Result<Vec<Models::Serie>, DbErr> {
    // Add an additional filter if we want non-explicit series only
    let mut conditions = Condition::all();
    if !explicit {
        conditions = conditions
            .add(Series::Column::Explicit.eq(0))
            .add(Series::Column::Valide.eq(1))
            .add(Series::Column::Lang.eq(lang))
    } else {
        conditions = conditions
            .add(Series::Column::Valide.eq(1))
            .add(Series::Column::Lang.eq(lang))
    }
        
    let series: Vec<Models::Serie> = Series::Entity::find()
        .filter(conditions)
        .limit(limit)
        .order_by(Series::Column::DateDebut, Order::Desc)
        .into_model::<Models::Serie>()
        .all(db)
        .await?;

    Ok(series)
}

///
/// Get a list of series that the user searched
/// 
/// ```
/// input -> string of the data input from the search bar by the user
/// explicit -> series with possible nudity
/// lang -> language of the device
/// ```
/// 
pub async fn get_series_by_search(input: String, explicit: bool, lang: String, db: &DatabaseConnection) -> Result<Vec<Models::Serie>, DbErr> {
    // Create our search string to be used in the query 
    let like_statement: &str = &format!("%{}%", input);

    // Build our conditions
    let mut conditions = Condition::all()
        .add(Series::Column::Valide.eq(1))
        .add(
            Condition::any()
                .add(Series::Column::Titre.like(like_statement))
                .add(Authors::Column::Nom.like(like_statement))
                .add(Genres::Column::Genre.like(like_statement))
        )
        .add(Series::Column::Lang.eq(lang))
        .add(Series::Column::Creator.eq(0));
        
    // Explicit flag, don't return possible nudity content
    if !explicit {
        conditions = conditions
            .add(Series::Column::Explicit.eq(0))
    }

    let series: Vec<Models::Serie> = Series::Entity::find()
        .join_rev( // Left join series_authors
            JoinType::LeftJoin,
            AuthorsSeries::Entity::belongs_to(Series::Entity)
                .from(AuthorsSeries::Column::SerieId)
                .to(Series::Column::IdSerie)
                .into()
        )
        .join_rev( // Left join authors
            JoinType::LeftJoin,
            Authors::Entity::belongs_to(AuthorsSeries::Entity)
                .from(Authors::Column::IdAuteur)
                .to(AuthorsSeries::Column::AuteurId)
                .into()
        )
        .join_rev( // Left join series_genres
            JoinType::LeftJoin,
            GenresSeries::Entity::belongs_to(Series::Entity)
                .from(GenresSeries::Column::SerieId)
                .to(Series::Column::IdSerie)
                .into()
        )
        .join_rev( // Left join genres
            JoinType::LeftJoin,
            Genres::Entity::belongs_to(GenresSeries::Entity)
                .from(Genres::Column::IdGenre)
                .to(GenresSeries::Column::GenreId)
                .into()
        )
        .filter(conditions)
        .group_by(Series::Column::IdSerie)
        .into_model::<Models::Serie>()
        .all(db)
        .await?;

    Ok(series)
}

///
/// Get a single serie by it's id
/// 
/// ```
/// id -> id of the wanted serie
/// ```
/// 
pub async fn get_serie_by_id(id: i32, db: &DatabaseConnection) -> Result<Option<Models::Serie>, DbErr> {
    let serie: Option<Models::Serie> = Series::Entity::find_by_id(id)
        .filter(Series::Column::Valide.contains("1"))
        .filter(Series::Column::Explicit.contains("0"))
        .into_model::<Models::Serie>()
        .one(db)
        .await?;

    Ok(serie)
}
