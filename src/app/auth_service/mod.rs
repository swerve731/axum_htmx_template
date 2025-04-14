use super::WebService;
pub mod templates;

pub mod error;
pub struct AuthService {}

impl WebService for AuthService {
    fn view_router(&self) -> axum::Router {
        axum::Router::new()
    }

    fn api_router(&self) -> axum::Router {
        axum::Router::new()
    }
}

impl AuthService {
    pub fn default() -> Self {
        AuthService {}
    }
}