/// Routing
/// 
/// This files contains all routes of the API
/// Refer to the swagger for some more information
/// 
/// Made with Hyper <https://docs.rs/hyper/0.14.17/hyper/>
/// _______________________________________________________
/// TODO: Logger to know what happened

/// Define the allowed methods, separated by a comma
const __ALLOWED_METHODS: &str = "GET, POST";

use crate::oauth::jwt as JWT;
use crate::router as Router;
use crate::controllers as Controllers;

use hyper::{Body, Method, Request, Response};
use sea_orm::DatabaseConnection;

/// 
/// This is our service handler.
/// ```
/// splits our uri path into segments and sends it to routing()
/// req -> the incomming Request<Body>
/// ```
/// 
pub async fn serve(req: Request<Body>, db: DatabaseConnection) -> Result<Response<Body>, hyper::Error> {
    // Only accept GET & POST
    if !matches!(req.method(), &Method::GET | &Method::POST) {
        return Ok(Router::responses::method_not_allowed(__ALLOWED_METHODS))
    }

    let jwt;
    match JWT::validate(&req).await {
        Ok(tkn) => jwt = tkn,
        Err(e) => {
            println!("{:?}", &e);
            return Ok(Router::responses::forbidden())
        }
    }

    if jwt.claims.azp == dotenv::var("API_KEY").unwrap() {
        println!("{:?}", jwt.claims);
        println!("{:?}", jwt.header);
        // TODO: Make some claims and compare them to the received
        // jwt and open specific access to the API
        //
        // atm we open it completely when the jwt is valid but we should do more checks
        
        // Extract segments from the URI path
        let path = req.uri().path().to_owned();
        let seg: Vec<&str> = path
            .split("/")
            .filter(
                |s| !s.is_empty()
            )
            .collect();
        println!("{:?}", seg);
        // Send to out routing
        routing(req, &seg, &db).await
    } else {
        println!("JWT Error => {:?}", &jwt);
        Ok(Router::responses::forbidden())
    }
}

///
/// Dispatch urls and actions accordingly
/// 
/// ```
/// req -> the incomming Request<Body>
/// seg -> the url segments passed as a &Vec<&str>
/// ```
/// 
async fn routing(req: Request<Body>, seg: &[&str], db: &DatabaseConnection) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), seg) {
        //
        // request path => /
        //
        (&Method::GET, []) => Ok(Response::new(Body::from("Default empty route, this will only return this message"))),

        //
        // request path => /api/series
        // 
        // Return a list of series
        // 
        (&Method::GET, ["api", "series"]) => Ok(Controllers::series::list(req, db).await),

        //
        // request path => /api/series/search
        //
        // Return a list of series from the search input
        // 
        // Pass the data encoded like so
        //      '{\"input\":\"some_user_input\"}'
        //
        (&Method::POST, ["api", "series", "search"]) => Ok(Controllers::series::list(req, db).await),

        //
        // request path => /api/serie/{id}
        //
        // Return the requested serie by it's id
        //
        (&Method::GET, ["api", "serie", id]) => Ok(Controllers::series::one(req, id.to_owned(), db).await),
        
        //
        // request path => /api/serie/{id}/firstEpisode
        // 
        // Return the requested first episode by a serie id
        // 
        (&Method::GET, ["api", "serie", id, "firstEpisode"]) => Ok(Controllers::episodes::one(req, id.to_owned(), "firstEpisode", db).await),

        //
        // request path => /api/serie/{id}/episode/{epid}
        //
        // Return the requested episode with the ids passed
        //
        (&Method::GET, ["api", "serie", id, "episode", epid]) => Ok(Controllers::episodes::one(req, id.to_owned(), epid.to_owned(), db).await),

        //
        // request path => /api/episode/{epid}/comments
        //
        // Return all comments related to the requested episode
        //
        (&Method::GET, ["api", "episode", epid, "comments"]) => Ok(Controllers::comments::episode(req, epid.to_owned(), db).await),

        //
        // request path => method not found response (404)
        //
        _ => Ok(Router::responses::not_found())
    }
}
