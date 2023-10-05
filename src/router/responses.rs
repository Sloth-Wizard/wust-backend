/// Handles all http responses
/// 
/// We serialize the data to output it right in here too
/// 
/// Use these as the Response<Body>
/// for the result of our route handler
/// 
/// Example usage in handler => `Ok(responses::ok_json())`
/// _______________________________________________________

use hyper::{Body, Response, StatusCode};
use hyper::header;

/// 
/// Simple OK response with default content-type
/// 
/// This function is mainly for testing first
/// so we add an underscore to avoid bloating
/// ourselves with warnings
/// 
pub fn _ok() -> Response<Body> {
    Response::builder()
    .status(StatusCode::OK)
    .header("wust", "In Rust We Trust")
    .body(Body::empty())
    .unwrap()
}

/// 
/// Return the data with a json header
/// 
/// Data needs to be serialized to a string before
/// sending it to the body
/// 
pub fn ok_json(data: String) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header("wust", "In Rust We Trust")
        .body(Body::from(data))
        .unwrap()
}

///
/// Return a json meaning that tells us
/// the data searchedf wasn't found
/// 
/// Pass a message to precise the issue 
/// and a path to know where it came from
/// 
pub fn not_ok_json(message: String, path: &str) -> Response<Body> {
    let start = "{\"destination\":";
    let middle = ",\"error\": true, \"message\":";
    let end = "}";

    Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(format!("{}\"{}\"{}\"{}\"{}", start, path, middle, message, end)))
        .unwrap()
}

/// 
/// Indicate that the method used is invalid
/// 
/// Use of header::ALLOW to display a list of valid
/// methods to be used
/// 
/// Try it out by starting your server and then
///     `curl -X PUT http://127.0.0.1:3000`
/// 
pub fn method_not_allowed<S: AsRef<str>>(methods: S) -> Response<Body> {
    Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .header(header::ALLOW, methods.as_ref())
        .body(Body::empty())
        .unwrap()
}

///
/// Indicates that the method hasn't been made yet
/// 
/// Pass the route requested to add as the method
/// requested into the returned json
/// 
pub fn _not_implemented(path: &str) -> Response<Body> {
    let start = "{\"destination\": \"unimplemented\",\"error\": true,\"method\":";
    let end = "}";

    Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(format!("{}\"{}\"{}", start, path, end)))
        .unwrap()
}

///
/// No request handler associated with a resource
/// 
pub fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}

///
/// Access forbidden
/// 
pub fn forbidden() -> Response<Body> {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .body(Body::empty())
        .unwrap()
}

///
/// Internal server error
/// 
/// For example, if the connexion to the
/// database failed
/// 
pub fn _internal_server_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .unwrap()
}

///
/// Internal server error with a message param
/// 
/// For dev uses atm
/// 
/// TODO: merge this function and _internal_server_error one with an Option<&str> parameter
/// 
pub fn _internal_server_error_message(error: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("{}", error)))
        .unwrap()
}
