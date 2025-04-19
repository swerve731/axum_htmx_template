use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use askama::Template;
use axum::{debug_handler, extract::{FromRequestParts, State}, response::{Html, IntoResponse}, Form};
use axum_extra::extract::cookie::Cookie;
use http::{header, request::Parts, StatusCode};
use jsonwebtoken::Validation;
use serde::{Deserialize, Serialize};


use crate::app::{
    auth_service::{api::jwt::KEYS, error::AuthError, templates::ChangePasswordTemplate}, db_service::user::{change_user_password, get_user_by_email}, utils::HxRedirect, AppState
};

use rand::prelude::*;



#[derive(Serialize, Deserialize, Debug)]
pub struct EmailAuthenticationClaims {
    user_id: String,
    exp: i64,
    code: String
}

impl EmailAuthenticationClaims {
    pub const EXPIRE_TIME_MINUTES: i64 = 15;
    pub const CODE_LENGTH: usize = 6;

    pub fn new(user_id: String) -> Self {
        let rng = rand::rng();
        let token: String  = rng.sample_iter(rand::distr::Alphanumeric).take(Self::CODE_LENGTH).map(|v|v.to_ascii_uppercase() as char).collect();
        let expiration_time = (chrono::Utc::now() + chrono::Duration::minutes(Self::EXPIRE_TIME_MINUTES)).timestamp();

        Self {
            user_id,
            exp: expiration_time,
            code: token
        }
    }
}

impl<S> FromRequestParts<S> for EmailAuthenticationClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookies= parts.headers.get(header::COOKIE);
        
        match cookies {
            Some(cookies) => {
                // parse the cookies and get the token
                let cookies_str = cookies.to_str().map_err(|_| AuthError::InvalidToken)?;
                let cookie_parts: Vec<&str> = cookies_str.split(';').collect();
                let mut token = None;
                for part in cookie_parts {
                    let trimmed_part = part.trim();
                    if trimmed_part.starts_with("token=") {
                        token = Some(trimmed_part[6..].to_string());
                        break;
                    }
                }

                let token = token.ok_or(AuthError::NoToken)?;

                tracing::debug!("Token: {:?}", token);

                let token_data = jsonwebtoken::decode(token.as_str(), &KEYS.decoding, &Validation::default())
                    .map_err(|e| {
                        tracing::debug!("Error decoding token: {:?}", e);
                        AuthError::InvalidToken
                    })?;
                return Ok(token_data.claims)

            },
            None => return Err(AuthError::NoToken)
        }

    }
}



#[derive(serde::Deserialize, Debug)]
pub struct EmailLogin {
    pub email: String
}

#[debug_handler]
pub async fn email_login(State(state): State<AppState>, Form(payload): Form<EmailLogin>) -> Result<impl IntoResponse, AuthError> {
    let smtp_service = state.smtp_service;

    // tracing::debug!("Payload: {:?}", payload);
    let user = get_user_by_email(&payload.email, &state.pool).await?;

    // tracing::debug!("User: {:?}", &user);
    if user.is_none() {
        return Err(AuthError::UserNotFound)
    }

    let user = user.unwrap();

    let token = EmailAuthenticationClaims::new(user.id.to_string());
    let code = token.code.clone();
    
    let body = format!("Your one time verification code is: {}\n Your code will expire in {} minutes", code, EmailAuthenticationClaims::EXPIRE_TIME_MINUTES);
    let message = smtp_service.create_message(body, user.email.clone(), user.name, "Email Code".to_string())?;
    smtp_service.send_message(message)?;

    let body =  super::super::templates::EmailCodeTemplate {
        email: user.email.clone(),
        expire_time: EmailAuthenticationClaims::EXPIRE_TIME_MINUTES.to_string()
    }.render()?;

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(), 
        &token, &KEYS.encoding
    )?;

    let cookie = Cookie::build(("token", token))
        .path("/")
        .http_only(true)
        .to_string();

    // let response = Response::builder()
    //     .header(header::SET_COOKIE, cookie) // Use the constant for clarity
    //     .body(Html(body))
    //     .unwrap();
    

    return Ok(
        (
            [(header::SET_COOKIE, cookie)],
            Html(body)
        )
    )


}

#[derive(serde::Deserialize)]
pub struct EmailCode {
    pub code: String
}

#[debug_handler]
pub async fn verify_email_code(claims: EmailAuthenticationClaims, Form(payload): Form<EmailCode>) -> Result<impl IntoResponse, AuthError> {

    if payload.code == claims.code {
        return Ok(
            ChangePasswordTemplate{}.into_response()
        )
        
    } else {
        return Err(AuthError::WrongPassword)
    }

}

#[derive(serde::Deserialize)]
pub struct ChangePassword {
    pub password: String,
    pub confirm_password: String
}

pub async fn change_password( claims: EmailAuthenticationClaims, State(state): State<AppState>, Form(payload): Form<ChangePassword>) -> Result<impl IntoResponse, AuthError> {
    if payload.password != payload.confirm_password {
        return Err(AuthError::PasswordsDontMatch)
    }

    let argon2 = argon2::Argon2::default();
    let password = payload.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(password, &salt)?.to_string();

    change_user_password(claims.user_id.parse().map_err(|_e| AuthError::InternalServer)?, &password_hash, &state.pool, claims).await?;

    Ok((
        [(HxRedirect::HEADER_NAME, "/dashboard")],
        StatusCode::OK
    ))

    
}
