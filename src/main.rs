use askama::Template;
use axum::{
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    let app = Router::new().route("/", get(home));

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Response {
    let html_string = HomeTemplate {}.render().unwrap();

    Html(html_string).into_response()
}

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate {}
