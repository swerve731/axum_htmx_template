use axum::response::{IntoResponse, Response};
use http::StatusCode;
use repo::error::RepoError;

pub mod features;
pub mod utils;
pub mod config;
pub mod repo;
pub mod web_service;
pub mod mailer;

#[derive(derive_more::From, Debug)]
pub enum ServerError {
    
    #[from]
    Tokio(tokio::io::Error),
    #[from]
    Axum(axum::Error),
    #[from]
    Askama(askama::Error),

    #[from]
    RepoError(RepoError)
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "An Internal Server Error Occurred, please try again later").into_response()
    }   
}