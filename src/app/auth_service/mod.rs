use api::email_login::{change_password, verify_email_code};
use axum::{response::IntoResponse, routing::{get, post}};

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
            .route("/email-login", get(|| async { templates::EmailLoginTemplate{}.into_response() }))
            .with_state(state.clone())
            .layer(super::App::cors_layer())
    }

    fn api_router(&self, state: super::AppState) -> axum::Router<super::AppState> {
        axum::Router::new()
            .route("/login", post(api::login_user))
            .route("/register", post(api::register_user))
            .route("/logout", post(api::logout_user))
            .route("/email-login", post(api::email_login::email_login))
            .route("/email-code", post(verify_email_code))
            .route("/change-password", post(change_password))
            .layer(super::App::cors_layer())
            .with_state(state.clone())
    }
}

impl AuthService {
    pub fn default() -> Self {
        AuthService {}
    }
}

