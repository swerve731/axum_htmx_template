use axum::response::{IntoResponse, Response};
use http::StatusCode;



#[derive(Debug, derive_more::From)]
pub enum ClaimsError {
    TokenNotFound,
    InvalidToken,
    #[from]
    JsonWebToken(jsonwebtoken::errors::Error)
}



impl IntoResponse for ClaimsError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidToken=> {
                return axum::response::Redirect::to("/auth/login").into_response()
            },
            Self::TokenNotFound => {
                return axum::response::Redirect::to("/auth/login").into_response()
                // (StatusCode::UNAUTHORIZED, [(HxRedirect::HEADER_NAME, Self::REDIRECT_URI)]).into_response()
            },
            // this is for all errors that can be considered internal server errors from crates or smthn
            err => {
                tracing::error!("A claims error occured: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error, please try again later.").into_response()
            }
        }
    }
}