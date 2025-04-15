

#[derive(Debug, derive_more::From)]
pub enum AppError {
    #[from]
    Auth(super::auth_service::error::AuthError),
    #[from]
    Axum(axum::Error),    
    #[from]
    Tokio(tokio::io::Error),
}


