use axum::routing::{ get, post };
use axum::Router;
use crate::controllers::{ auth, dashboard };
use crate::db::create_pool;

pub fn create_routes() -> Router {
    let pool = create_pool();
    Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/", get(auth::login_form))
        .route("/register", get(auth::register_form))
        .route("/dashboard", get(dashboard::dashboard))
        
        
}
