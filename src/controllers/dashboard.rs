use axum::extract::Form;
use axum::response::{Html, Redirect};
use askama::Template;

#[derive(Template)]
#[template(path = "customer/dashboard/index.html")]
struct DashboardTemplate;

pub async fn dashboard() -> Html<String> {
    let template = DashboardTemplate {};
    Html(template.render().unwrap())
}