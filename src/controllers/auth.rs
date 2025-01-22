
use askama::Template;
use axum::response::IntoResponse;
use axum::Json;
use crate::models::auth::{LoginForm, RegisterForm};
use crate::models::user::NewUser;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::schema::users::dsl::*;
use crate::db::create_pool;
use axum::{
    extract::{Form, State},
    response::{Html, Redirect},
};
use tokio::task;
use axum_macros::debug_handler;

// Templates
#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate;



// Handlers
pub async fn login_form() -> Html<String> {
    let template = LoginTemplate {};
    Html(template.render().unwrap())
}

pub async fn register_form() -> Html<String> {
    let template = RegisterTemplate {};
    Html(template.render().unwrap())
}

pub async fn login(Form(form): Form<LoginForm>) -> Redirect {
    println!("Username: {}, Password: {}", form.username, form.password);
    Redirect::to("/")
}


#[debug_handler]
pub async fn register(
    Form(form): Form<RegisterForm>
   
)
-> impl IntoResponse {
let hashed_password=match hash(&form.password, DEFAULT_COST) {
    Ok(h) => h,
    Err(e) => return Err(format!("Error hashing password: {}", e)),
};
// // Create a new user
let new_user = NewUser {
    username: form.username,
    email: form.email,
    password: hashed_password,
    role: "user".to_string(),
    created_at: Some(Utc::now().naive_utc()),
    updated_at: None,
    deleted_at: None,
};
// Get a connection from the pool
// Get a database connection from the pool
let mut conn = create_pool();

// Insert the new user into the database
diesel::insert_into(users)
    .values(&new_user)
    .execute(&mut conn)
    .map_err(|e| format!("Failed to insert new user: {}", e))?;

Ok(Json("User created successfully"))
} 
    


