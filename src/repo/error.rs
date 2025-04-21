use axum::response::{Html, IntoResponse, Response};
use http::StatusCode;



#[derive(derive_more::From, Debug)]
pub enum RepoError {  
    ValidationError {
        body: Html<String>
    },
    EmailNotFound,

    #[from]
    Sqlx(sqlx::Error),

    #[from]
    Argon2(argon2::Error),

    #[from]
    PasswordHashing(argon2::password_hash::Error),
    
    #[from]
    Askama(askama::Error),
}

impl IntoResponse for RepoError {
    fn into_response(self) -> Response {
        match self {
            RepoError::EmailNotFound => {
                (StatusCode::NOT_FOUND, "Email not found").into_response()
            },
            RepoError::ValidationError { body } => {
                (StatusCode::BAD_REQUEST, body).into_response()
            },
            err => {
                tracing::error!("REPOSITORY: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "An Internal Server Error Occurred, please try again later").into_response()
            }
        }
    }   
}