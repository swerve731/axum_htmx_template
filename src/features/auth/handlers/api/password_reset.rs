use askama::Template;
use axum::{body::Body, extract::State, response::IntoResponse, Form};
use http::{header::SET_COOKIE, StatusCode};
use serde::Deserialize;

use crate::{features::auth::{claims::{password_reset::PasswordResetClaim, Claims}, error::AuthError, handlers::views::password_reset::{CodeForm, ResetPasswordForm}}, repo::infra::user::UserRepo, utils::HxRedirect, web_service::server::ServerState};


#[derive(Template)]
#[template(path = "auth/emails/one_time_code.html")]
pub struct OneTimeCodeEmailTemplate {
    code: String,
    expire_time: String,
}

#[derive(Deserialize)]
pub struct EmailPayload {
    email: String,
}

//this returns the code form
pub async fn email_code(State(state): State<ServerState>, Form(payload): Form<EmailPayload>) -> Result<impl IntoResponse, AuthError> {
    let user = state.repo
        .user_get_by_email(payload.email.as_str())
        .await?
        .ok_or(AuthError::EmailNotFound)?;

    let token = PasswordResetClaim::new(payload.email.clone());
    let code = token.code.clone();
    
    let body = OneTimeCodeEmailTemplate {
        code,
        expire_time: PasswordResetClaim::EXPIRE_TIME_MINUTES.to_string(),
    }
    .render()?;

    let message = state.mailer.create_message(body, user.email.clone(), user.name, "Email Code".to_string())?;

    state.mailer.send_message(message)?;

    let body = CodeForm{
        email: payload.email,
        expire_time: PasswordResetClaim::EXPIRE_TIME_MINUTES.to_string(),
    }.render()?;

    let cookie = token.cookie()?;
    Ok(
        (
            [(SET_COOKIE, cookie)],
            Body::from(body)
        ).into_response()
    )
}


#[derive(Deserialize)]
pub struct CodeLoginPayload {
    code: String,
}

pub async fn code_login(claim: PasswordResetClaim ,Form(payload): Form<CodeLoginPayload>) -> Result<impl IntoResponse, AuthError> {
    let mut claim = claim.clone();

    if !claim.authorize(payload.code.as_str()) {
        return Err(AuthError::WrongPassword);
    } 

    let body = ResetPasswordForm {}.render()?;

    tracing::debug!("TOKEN: {:?}", claim);
    let cookie = claim.cookie()?;


    Ok((
        [(SET_COOKIE, cookie)],
        Body::from(body)
    ).into_response())
}

#[derive(Deserialize)]
pub struct ChangePasswordPayload {
    password: String,
    confirm_password: String,
}

pub async fn change_password(State(state): State<ServerState>, claim: PasswordResetClaim, Form(payload): Form<ChangePasswordPayload>) -> Result<impl IntoResponse, AuthError> {
    if !claim.authorized {
        return Ok((
            [(HxRedirect::HEADER_NAME, "/")],
            StatusCode::UNAUTHORIZED,
        ).into_response());
    }
    if payload.password != payload.confirm_password {
        return Err(AuthError::PasswordsDontMatch);
    }
    let user = state.repo
        .user_get_by_email(claim.email.as_str())
        .await?;

    if user.is_none() {
        return Ok((
            [(HxRedirect::HEADER_NAME, "/")],
            StatusCode::UNAUTHORIZED,
        ).into_response());
    }
    let user = user.unwrap();
    

    let _res = state.repo.user_change_password(user.id, payload.password, claim).await?;
    

    Ok((
        [(HxRedirect::HEADER_NAME, "/auth/login")],
        StatusCode::OK,
    ).into_response())
}