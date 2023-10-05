pub mod make {
    use crate::new_status;
    // use crate::database as Db;
    use crate::models::episodes as Models;
    use sea_orm::DatabaseConnection;

    ///
    /// Get and add more data to the initial Episode struct
    /// 
    /// ```
    /// episode -> the data of a previous query on the webtoons table
    /// ```
    /// 
    /// TODO: Check session data if user has liked the episode or followed the episode serie
    /// 
    pub async fn one(episode: Models::Episode, _db: &DatabaseConnection) -> Models::EnrichedEpisode {
        let liked: bool = false;
        let followed: bool = false;
        Models::EpisodeProcessData::process(&episode, liked, followed, new_status!(episode.date_debut, chrono::NaiveDateTime))
    }
}
