use askama::Template;
use axum::{extract::State, response::{Html, IntoResponse}};
use http::StatusCode;

use crate::{features::auth::claims::authorization::AuthorizationClaim, repo::infra::user::UserRepo, web_service::server::ServerState, ServerError};



#[derive(Template)]
#[template(path = "user/dashboard.html")]
pub struct DashboardTemplate {
    email: String,
    name: String

}


impl DashboardTemplate {

    pub async fn handler(State(state): State<ServerState>, claim: AuthorizationClaim) -> Result<impl IntoResponse, ServerError> {
        let user = state.repo.user_get_by_id(claim.user_id).await?;

        if user.is_none() {
            return Ok((
                [(http::header::LOCATION, "/auth/login")],
                StatusCode::UNAUTHORIZED
            ).into_response());
        }

        let user = user.unwrap();

        Ok((
            StatusCode::OK,
            Html(Self {
                email: user.email,
                name: user.name
            }
            .render()?)
        ).into_response())
    }
}


#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
}

impl IndexTemplate {
    pub async fn handler() -> Result<impl IntoResponse, ServerError> {
        Ok((
            StatusCode::OK,
            Html(Self {}.render()?)
        ).into_response())
    }
}