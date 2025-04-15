use axum::extract::State;

use crate::app::db_service::user::get_user_by_id;

use super::{auth_service::api::jwt::Claims, db_service::user::User, AppState, WebService};

pub mod templates;


pub struct UserService {}

impl WebService for User {
    fn view_router(&self, state: AppState) -> axum::Router<AppState> {
        axum::Router::new()
            .with_state(state.clone())
    }

    fn api_router(&self, state: AppState) -> axum::Router<AppState> {
        axum::Router::new()
            .with_state(state.clone())
    }
}



pub async fn user_dashboard(claims: Claims, State(state): State<AppState>) -> Result<impl IntoResponse, crate::Error> {
    let user = get_user_by_id(claims.user_id, &state.pool).await?;
    

    todo!("User dashboard logic here");
}