use axum::response::{IntoResponse, Response};
use http::StatusCode;

#[derive(derive_more::From, Debug)]
pub enum SmtpError {
    #[from]
    Lettre(lettre::error::Error),
    #[from]
    Transport(lettre::transport::smtp::Error),
    #[from]
    Address(lettre::address::AddressError)
}


impl IntoResponse for SmtpError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self { 
            SmtpError::Lettre(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            SmtpError::Transport(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            SmtpError::Address(err) => (StatusCode::BAD_REQUEST, err.to_string()),
        };

        (status, error_message).into_response()
    }
    
}