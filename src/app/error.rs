use axum::response::{IntoResponse, Response};
use http::StatusCode;


#[derive(Debug, derive_more::From)]
pub enum AppError {
    #[from]
    Auth(super::auth_service::error::AuthError),
    #[from]
    Axum(axum::Error),    
    #[from]
    Tokio(tokio::io::Error),
    #[from]
    Sqlx(sqlx::Error),
    #[from]
    Askama(askama::Error),
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        
        match self {
            
            AppError::Auth(e) => e.into_response(),
            AppError::Axum(e) => {
                tracing::error!("Axum error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
            AppError::Tokio(e) => {
                tracing::error!("Tokio error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
            AppError::Sqlx(e) => {
                tracing::error!("SQLx error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            },
            AppError::Askama(e) => {
                tracing::error!("Askama error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}


