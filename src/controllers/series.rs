/// 
/// All series related controllers
/// 
use crate::database as Db;
use crate::router as Router;

use hyper::{Body, Method, Request, Response};
use serde::Deserialize;
use sea_orm::DatabaseConnection;

use super::helpers::series_helper::make;

///
/// Prepare the response for series list endpoints
/// 
/// ```
/// req -> the incomming Request<Body>
/// ```
/// 
pub async fn list(req: Request<Body>, db: &DatabaseConnection) -> Response<Body> {
    let series = match req.method() {
        &Method::GET => { // Default list response
            Db::series::get_series(16, false, String::from("fr"), db).await
        },
        &Method::POST => { // List from search input
            #[derive(Deserialize, Debug)]
            struct ReqPost {
                input: String
            }

            // Get the body of the POST request as bytes
            let bytes = &hyper::body::to_bytes(req.into_body())
                .await
                .unwrap();
            // Put those bytes into a struct
            let res: ReqPost = serde_json::from_slice(&bytes).unwrap();
            // Then go get the searched series and return
            Db::series::get_series_by_search(res.input, false, String::from("fr"), db).await
        },
        // Usage of unauthoriwed method err
        _ => Err(sea_orm::DbErr::Custom("Unauthorized method somehow got past the check (WUST-418rfg4z8d4fgz)".to_string()))
    };

    match series {
        Ok(series) => {
            // If we have a serie, we can get the authors and genres
            let result = make::list(series, db).await;

            Router::responses::ok_json(serde_json::to_string(&result).unwrap())
        }
        Err(e) => return Router::responses::not_ok_json(e.to_string(), "Series list controllers (WUST-D1f6s51df98gqw)")
    }
}


///
/// Prepare the response for a single serie
/// 
/// ```
/// req -> the incomming Request<Body>
/// id -> id of the serie
/// ```
/// 
pub async fn one(req: Request<Body>, id: &str, db: &DatabaseConnection) -> Response<Body> {
    // pid means parsed id because we have to
    // parse a value gotten from an url param
    let pid: i32 = id.parse::<i32>().unwrap();
    let serie = Db::series::get_serie_by_id(pid, db)
        .await;

    match serie {
        Ok(serie) => {
            // If we have a serie, we can get the authors, genres and episodes list
            // This starts the process to push all the needed data into the SerieJsonResponse struct
            let result = make::one(serie.unwrap(), db).await;

            // --------------------------------------------------------------------------
            // Our main return
            Router::responses::ok_json(serde_json::to_string(&result).unwrap())
            // --------------------------------------------------------------------------
        },
        Err(e) => return Router::responses::not_ok_json(e.to_string(), req.uri().path())
    }
}
