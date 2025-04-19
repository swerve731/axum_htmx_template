pub mod jwt;
pub mod utils;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{body::Body, extract::{Form, State}, response::{IntoResponse, Response}};
use http::Uri;
use jwt::{AuthResponse, Claims, KEYS};
use utils::{is_valid_email, is_valid_password};

use crate::app::{db_service::user::get_user_by_email, utils::HxRedirect, AppState};
pub mod email_login;

use super::error::AuthError;


#[derive(serde::Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
}





pub async fn register_user(State(state): State<AppState>, Form(user_data): Form<RegisterUser>) -> Result<impl IntoResponse, AuthError> {
    is_valid_email(&user_data.email)?;
    is_valid_password(&user_data.password)?;

    let user_res = get_user_by_email(&user_data.email, &state.pool).await?;
    
    if user_res.is_some() {
        tracing::debug!("User already exists");
        return Err(AuthError::EmailAlreadyExists)
    } else {
        tracing::debug!("User does not exist, creating user...");

        let argon2 = argon2::Argon2::default();
        let password = user_data.password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2.hash_password(password, &salt)?.to_string();

        let id = sqlx::types::uuid::Uuid::new_v4();



        let _res = sqlx::query!(
            "INSERT INTO users (id, name, email, password_hash) VALUES ($1, $2, $3, $4)",
            id,
            user_data.name,
            user_data.email,
            password_hash
        )
        .execute(&state.pool)
        .await?;

        let claims = Claims::new(id.to_string());

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(), 
            &claims, &KEYS.encoding
        )?;

        Ok(AuthResponse {
            token,
            redirect_uri: Uri::from_static("/dashboard")
        })
    }
}   



#[derive(serde::Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

pub async fn login_user(State(state): State<AppState>, Form(user_data): Form<LoginUser>) -> Result<impl IntoResponse, AuthError> {
    // is_valid_email(&user_data.email)?;

    let user = match get_user_by_email(&user_data.email, &state.pool).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AuthError::UserNotFound),
        Err(_) => return Err(AuthError::Sqlx(sqlx::Error::RowNotFound)),
    };
    
    
    let correct_password = user.password_hash;
    let password = user_data.password.as_bytes();
    let parsed_hash = PasswordHash::new(&correct_password)?;
    let res = Argon2::default().verify_password(password, &parsed_hash);

    
    match res {
        Ok(_) => {
            let claims = Claims::new(user.id.clone().to_string());
        
            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(), 
                &claims, &KEYS.encoding
            )?;
            
            return Ok(AuthResponse {
                token,
                redirect_uri: Uri::from_static("/dashboard")
            });
        },
        Err(e) => match e {
            argon2::password_hash::Error::Password => return Err(AuthError::WrongPassword),
            _ => return Err(AuthError::PasswordHashing(e)),
        },
        
    }
}

pub async fn logout_user() -> Result<impl IntoResponse, AuthError> {
    Ok(
        Response::builder()
            .header(HxRedirect::HEADER_NAME, "/")
            .header(http::header::SET_COOKIE, "token=; Path=/; HttpOnly")
            .body(Body::empty())
            .unwrap()
    )
}