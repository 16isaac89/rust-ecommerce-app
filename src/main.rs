
use axum::{
    extract::{Form, Query},
    response::{Html, Redirect},
    routing::{get, post},
    Router,
};
use askama::Template;
use std::net::SocketAddr;

mod routes;
mod models;


use routes::auth;

#[tokio::main]
async fn main() {
    // Define the routes
    let app = Router::new()
        .route("/", get(auth::login_form))
        .route("/register", get(auth::register_form))
        .route("/login", post(auth::login))
        .route("/register", post(auth::register));

    // Run the Axum server
    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum_server::bind(addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

