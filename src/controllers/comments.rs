///
/// All comments related controllers
/// 
use crate::database as Db;
use crate::router as Router;

use hyper::{Body, Method, Request, Response};
use serde::Deserialize;
use sea_orm::DatabaseConnection;

///
/// Get all comments for a single episode
/// 
/// ```
/// req -> the incomming Request<Body>
/// epid -> id of the episode
/// ```
/// 
pub async fn episode(req: Request<Body>, epid: &str, db: &DatabaseConnection) -> Response<Body> {
    let pepid: i32 = epid.parse::<i32>().unwrap();

    let comments = Db::comments::get_comments(pepid, 3, db).await;
    
    match comments {
        Ok(comments) => {
            // --------------------------------------------------------------------------
            // Our main return
            Router::responses::ok_json(serde_json::to_string(&comments).unwrap())
            // --------------------------------------------------------------------------
        },
        Err(e) => return Router::responses::not_ok_json(e.to_string(), req.uri().path())
    }
}
