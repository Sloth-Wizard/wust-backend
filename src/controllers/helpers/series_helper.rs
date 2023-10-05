pub mod make {
    use crate::new_status;
    use crate::database as Db;
    use crate::models::series as Models;
    use sea_orm::DatabaseConnection;

    ///
    /// Constructs a list of series
    /// 
    /// ```
    /// series -> the data of a previous query on the series table
    /// db -> the cloned connection pool
    /// ```
    /// 
    pub async fn list(series: Vec<Models::Serie>, db: &DatabaseConnection) -> Vec<Models::SeriesListJsonResponse> {
        // Prepare a new Vec to push data later
        let mut result: Vec<Models::SeriesListJsonResponse> = Vec::new();
        // Start building the SeriesListJsonResponse
        // struct for each serie in the series Vec
        for serie in series {
            // Query the authors table
            let authors = Db::authors::get_authors_by_serie_id(serie.id_serie, db)
                .await
                .unwrap();

            // Query the genres table
            let genres = Db::genres::get_genres_by_serie_id(serie.id_serie, db)
                .await
                .unwrap();

            let last_ep = Db::episodes::get_last_episode_date_by_serie_id(serie.id_serie, db)
                .await
                .unwrap();

            // Add all that data to the struct
            result.push(Models::SeriesListJsonResponse {
                serie: Models::SerieProcessData::process(&serie, new_status!(last_ep.unwrap().date_debut, chrono::NaiveDateTime)),
                authors: authors,
                genres: genres
            });
        }

        result
    }

    ///
    /// Constructs a single serie
    /// 
    /// ```
    /// serie -> the data of a previous query on the series table
    /// ```
    /// 
    pub async fn one(serie: Models::Serie, db: &DatabaseConnection) -> Models::SerieJsonResponse {
        // Query the authors table
        let authors = Db::authors::get_authors_by_serie_id(serie.id_serie, db)
            .await
            .unwrap();

        // Query the genres table
        let genres = Db::genres::get_genres_by_serie_id(serie.id_serie, db)
            .await
            .unwrap();

        // Query the episode table (webtoons)
        let episodes = Db::episodes::get_episodes_by_serie_id(serie.id_serie, db)
            .await
            .unwrap();

        // Add all that data to the struct and return it
        Models::SerieJsonResponse {
            serie: Models::SerieProcessData::process(&serie, new_status!(episodes.last().unwrap().date_debut, chrono::NaiveDateTime)),
            authors: authors,
            genres: genres,
            episodes: episodes
        }
    }
}
