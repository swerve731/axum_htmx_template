use axum::response::{IntoResponse, Response};
use http::StatusCode;



#[derive(derive_more::From, Debug)]
pub enum MailerError {
    #[from]
    Lettre(lettre::error::Error),
    #[from]
    Transport(lettre::transport::smtp::Error),
    #[from]
    Address(lettre::address::AddressError)
}

impl IntoResponse for MailerError {
    fn into_response(self) -> Response {
        tracing::error!("MESSAGING: an error occured: {:?}", self);
        match self { 
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "An error occured sending an email").into_response()
        }

    }
    
}