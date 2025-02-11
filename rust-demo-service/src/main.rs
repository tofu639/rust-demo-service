use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;

mod handlers;

#[tokio::main]
async fn main() {
    // Build our application with routes
    let app = Router::new()
        .route("/", get(handlers::hello_world))
        .route("/echo", post(handlers::echo))
        .layer(TraceLayer::new_for_http());

    // Define the address to run the server on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    // Run the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}