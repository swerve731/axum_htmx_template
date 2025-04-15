pub mod jwt;
pub mod utils;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::{Form, State}, Json};
use jwt::{AuthBody, Claims, KEYS};
use utils::{is_valid_email, is_valid_password};

use crate::app::{db_service::user::get_user_by_email, AppState};

use super::error::AuthError;



#[derive(serde::Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
}




pub async fn register_user(State(state): State<AppState>, Form(user_data): Form<RegisterUser>) -> Result<Json<AuthBody>, AuthError> {
    is_valid_email(&user_data.email)?;
    is_valid_password(&user_data.password)?;

    let argon2 = argon2::Argon2::default();
    let password = user_data.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

    let id = sqlx::types::uuid::Uuid::new_v4();

    if get_user_by_email(&user_data.email, &state.pool).await?.is_some() {
        return Err(AuthError::EmailAlreadyExists);
    }

    let _res = sqlx::query!(
        "INSERT INTO users (id, name, email, password_hash) VALUES ($1, $2, $3, $4)",
        id,
        user_data.name,
        user_data.email,
        password_hash
    )
    .execute(&state.pool)
    .await?;

    let claims = Claims {
        user_id: id.to_string(),
        exp: Claims::EXP_TIME,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(), 
        &claims, &KEYS.encoding
    )?;
    

    Ok(
        Json(
            AuthBody::new(token)
        )
    )
}   



#[derive(serde::Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

pub async fn login_user(State(state): State<AppState>, Form(user_data): Form<LoginUser>) -> Result<Json<AuthBody>, AuthError> {
    is_valid_email(&user_data.email)?;

    let user = get_user_by_email(&user_data.email, &state.pool).await?.unwrap_or(
        return Err(AuthError::UserNotFound)
    );

    
    let correct_password = user.password_hash;
    let password = user_data.password.as_bytes();
    let parsed_hash = PasswordHash::new(&correct_password)?;
    let res = Argon2::default().verify_password(password, &parsed_hash);

    match res {
        Ok(_) => { 
            let claims = Claims {
                user_id: user.id.to_string(),
                exp: Claims::EXP_TIME,
            };
        
            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(), 
                &claims, &KEYS.encoding
            )?;
            
            return Ok(
                Json(
                    AuthBody::new(token)
                )
            );
        },
        Err(err) => { 
            match err {
                argon2::password_hash::Error::Password => {
                    return Err(AuthError::WrongPassword);
                },
                _ => {
                    return Err(AuthError::PasswordHashing(err));
                }
            }
         }
            
    }
}




