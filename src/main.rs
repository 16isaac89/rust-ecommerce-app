
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
mod controllers;
mod db;
mod schema;


use routes::web_routes;
use crate::db::create_pool;


#[tokio::main]
async fn main() {
    // Initialize the database pool
    create_pool();
    // Initialize routes from the routes module
     let app = routes::web_routes::create_routes();

     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
     println!("Server running at http://{}", addr);
 
     axum_server::bind(addr)
         .serve(app.into_make_service())
         .await
         .unwrap();
}

