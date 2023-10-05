use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use dotenv;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

///
/// Init internal crates
/// 
mod oauth;
mod router;
mod database;
mod entity;
mod models;
mod controllers;
mod macros;

/// 
/// Graceful shutdown
/// 
async fn shutdown_signal() {
    // Wait for CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

///
/// Start the API
/// 
#[tokio::main]
async fn main() -> Result<()> {
    // Change the .env filename according to your env
    // Use dotenv::dotenv().ok(); for production
    dotenv::from_filename(".env.local").ok();
    pretty_env_logger::init();

    // Share the DatabaseConnection with every service
    let db = database::connect::db::new()
        .await
        .unwrap();

    let addr = format!(
        "{}:{}",
        dotenv::var("LH_DNS").unwrap(),
        dotenv::var("LH_PORT").unwrap()
    ).parse()
    .expect("address creation works");
    
    let service = make_service_fn(move |_| {
        // Move a clone of `db` into the `service_fn`
        let db = db.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                // Clone again to ensure that db outlives this closure
                router::handler::serve(req, db.to_owned())
            }))
        }
    });
    let server = Server::bind(&addr).serve(service);
    let shutdown = server.with_graceful_shutdown(shutdown_signal());

    println!("Listening on http://{}", addr);

    // Run this server for... forever!
    if let Err(e) = shutdown.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
