use std::str::FromStr;

use askama::Template;
use axum::{extract::State, response::Html};
use sqlx::types::Uuid;

use crate::app::{auth_service::error::AuthError, db_service::user::get_user_by_id};

use super::{auth_service::api::jwt::Claims, AppState, WebService};

pub mod templates;


pub struct UserService {}

impl WebService for UserService {
    fn view_router(&self, state: AppState) -> axum::Router<AppState> {
        axum::Router::new()
            .route("/dashboard", axum::routing::get(user_dashboard))
            .with_state(state.clone())
    }

    fn api_router(&self, state: AppState) -> axum::Router<AppState> {
        axum::Router::new()
            .with_state(state.clone())
    }
}



pub async fn user_dashboard(claims: Claims, State(state): State<AppState>) -> Result<impl axum::response::IntoResponse, super::error::AppError> {
    
    let user_id = Uuid::from_str(&claims.user_id)
        .map_err(|_| AuthError::InvalidToken)?;

    let user = get_user_by_id(user_id, &state.pool).await?
        .ok_or(AuthError::UserNotFound)?;

    let view = templates::UserDashboard {
        email: user.email,
        name: user.name,
    };

    Ok(Html(view.render()?))
}