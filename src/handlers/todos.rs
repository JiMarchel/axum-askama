use crate::models::templates::{CreateTemplate, TodosTemplate};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

pub async fn todos_handler() -> Response {
    let html_string = TodosTemplate {}.render().unwrap();

    Html(html_string).into_response()
}

pub async fn create_todo_handler() -> Response {
    let html_string = CreateTemplate {}.render().unwrap();

    Html(html_string).into_response()
}
