use crate::{data::errors::DataError, models::templates};
use askama::Template;
use axum::{
    body::Body,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] DataError),

    #[error("Template error")]
    Template(#[from] askama::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let (status, response) = match self {
            AppError::Database(e) => server_error(e.to_string()),
            AppError::Template(e) => server_error(e.to_string()),
        };
        (status, response).into_response()
    }
}

fn server_error(e: String) -> (StatusCode, Response<Body>) {
    tracing::error!("Server error: {}", e);

    let html_string = templates::ServerErrorTemplate {}.render().unwrap();

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Html(html_string).into_response(),
    )
}
