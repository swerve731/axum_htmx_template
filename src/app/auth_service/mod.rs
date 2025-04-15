use axum::{response::IntoResponse, routing::get};

use super::WebService;
pub mod templates;
pub mod error;
pub mod api;

#[derive(Clone)]
pub struct AuthService {}

impl WebService for AuthService {
    fn view_router(&self, state: super::AppState) -> axum::Router<super::AppState> {
        axum::Router::new()
            .route("/login", get(|| async { templates::LoginTemplate{}.into_response() }))
            .route("/register", get(|| async { templates::RegisterTemplate{}.into_response() }))
            .with_state(state.clone())
    }

    fn api_router(&self, state: super::AppState) -> axum::Router<super::AppState> {
        axum::Router::new()
            .route("/login", get(api::login_user))
            .route("/register", get(api::register_user))
            .with_state(state.clone())
    }
}

impl AuthService {
    pub fn default() -> Self {
        AuthService {}
    }
}

