use axum::{routing::get, Router};

use crate::features::user::handlers::views::{DashboardTemplate, IndexTemplate};

use super::WebService;


pub struct UserService;

impl WebService for UserService {
    fn view_router(state: super::server::ServerState) -> axum::Router<super::server::ServerState> {
        Router::new()
            .route("/", get(IndexTemplate::handler))
            .route("/dashboard", get(DashboardTemplate::handler))
            .with_state(state)
    }

    fn api_router(state: super::server::ServerState) -> axum::Router<super::server::ServerState> {
        Router::new()
            .with_state(state)
    }
}