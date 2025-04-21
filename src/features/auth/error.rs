use axum::response::{IntoResponse, Response};
use http::StatusCode;

use crate::repo::error::RepoError;

use super::claims::error::ClaimsError;


#[derive(Debug, derive_more::From)]
pub enum AuthError {

    // for not valid email/password or passwords dont match etc...    
    EmailNotFound,
    EmailAlreadyExists,
    WrongPassword,
    PasswordsDontMatch,
    #[from]
    ClaimsError(ClaimsError),

    #[from]
    Askama(askama::Error),

    #[from]
    RepoError(RepoError),

    #[from]
    Mailer(crate::mailer::error::MailerError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::PasswordsDontMatch => {
                (StatusCode::BAD_REQUEST, "Passwords dont match").into_response()
            },
            AuthError::EmailNotFound => {
                (StatusCode::NOT_FOUND, "Email not found").into_response()
            },
            AuthError::EmailAlreadyExists => {
                (StatusCode::CONFLICT, "Email already exists, try signing in").into_response()
            },
            
            AuthError::ClaimsError(err) => {
                err.into_response()
            },
            
            AuthError::RepoError(err) => {
                err.into_response()
            },

            AuthError::WrongPassword => {
                (StatusCode::UNAUTHORIZED, "Wrong password").into_response()
            },

            err => {
                tracing::error!("A auth error occured: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error, please try again later.").into_response()
            }
        
        }
    }
}