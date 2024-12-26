use super::errors::AppError;
use crate::models::templates::HomeTemplate;
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

pub async fn home() -> Result<Response, AppError> {
    let html_string = HomeTemplate {}.render().unwrap();

    Ok(Html(html_string).into_response())
}
