
use crate::models::auth::{LoginForm, RegisterForm};
use axum::{
    extract::Form,
    response::{Html, Redirect},
};
use serde::Deserialize;
use askama::Template;


#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate;



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

pub async fn register(Form(form): Form<RegisterForm>) -> Redirect {
    println!(
        "Username: {}, Password: {}, Email: {}",
        form.username, form.password, form.email
    );
    Redirect::to("/")
}
