use axum::{extract::{Form, State}, response::IntoResponse};
use http::StatusCode;
use crate::{features::auth::error::AuthError, repo::infra::user::UserRepo, utils::HxRedirect, web_service::server::ServerState};


use crate::features::auth::claims::authorization::AuthorizationClaim;


pub async fn logout() -> Result<impl IntoResponse, AuthError> {
    Ok((
        [(http::header::SET_COOKIE, "token=; Path=/; HttpOnly")],
        [(HxRedirect::HEADER_NAME, "/")],
        StatusCode::OK,
    ))
}

#[derive(serde::Deserialize)]
pub struct RegisterPayload {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn register(State(state): State<ServerState>, Form(user_data): Form<RegisterPayload>) -> Result<impl IntoResponse, AuthError> {
    let user_res = state.repo.user_get_by_email(&user_data.email).await?; 
    if user_res.is_some() {
        tracing::debug!("User already exists");
        return Err(AuthError::EmailAlreadyExists)
    } else {
        tracing::debug!("User does not exist, creating user...");

        let id = state.repo.user_create(&user_data.email, &user_data.password, &user_data.name).await?;

        let claims = AuthorizationClaim::new(id);

        Ok(claims)
    }
}   



#[derive(serde::Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub async fn login(State(state): State<ServerState>, Form(user_data): Form<LoginPayload>) -> Result<impl IntoResponse, AuthError> {
    // is_valid_email(&user_data.email)?;

    let password_check = state.repo.user_check_password(&user_data.email, &user_data.password).await?;
    match password_check {
        Some(id) => {
            let claims = AuthorizationClaim::new(id);
            Ok(claims)
        },
        None => Err(AuthError::WrongPassword)
    }
}

// pub async fn logout_user() -> Result<impl IntoResponse, AuthError> {
//     Ok(
//         Response::builder()
//             .header(HxRedirect::HEADER_NAME, "/")
//             .header(http::header::SET_COOKIE, "token=; Path=/; HttpOnly")
//             .body(Body::empty())
//             .unwrap()
//     )
// }