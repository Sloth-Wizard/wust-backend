/// 
/// All episodes related controllers
/// 
use crate::database as Db;
use crate::router as Router;

use hyper::{Body, Request, Response};
use sea_orm::DatabaseConnection;

use super::helpers::episodes_helper::make;

///
/// Prepare the response for a single episode
/// 
/// ```
/// req -> the incomming Request<Body>
/// id -> id of the serie
/// epid -> id of the episode
/// ```
/// 
pub async fn one(req: Request<Body>, id: &str, epid: &str, db: &DatabaseConnection) -> Response<Body> {
    let episode;
    let pid: i32 = id.parse::<i32>().unwrap();

    if epid == "firstEpisode" {
        episode = Db::episodes::get_episode_first_by_serie_id(pid, db)
            .await;
    } else {
        let pepid: i32 = epid.parse::<i32>().unwrap();
        episode = Db::episodes::get_episode_by_ids(pid, pepid, db)
            .await;
    }

    match episode {
        Ok(episode) => {
            let result = make::one(episode.unwrap(), db).await;
            // --------------------------------------------------------------------------
            // Our main return
            Router::responses::ok_json(serde_json::to_string(&result).unwrap())
            // --------------------------------------------------------------------------
        },
        Err(e) => return Router::responses::not_ok_json(e.to_string(), req.uri().path())
    }
}
