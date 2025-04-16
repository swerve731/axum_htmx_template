pub mod jwt;
pub mod utils;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::{Form, State}, response::IntoResponse, Json};
use http::{header, StatusCode};
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




pub async fn register_user(State(state): State<AppState>, Form(user_data): Form<RegisterUser>) -> Result<impl IntoResponse, AuthError> {
    is_valid_email(&user_data.email)?;
    is_valid_password(&user_data.password)?;

    let argon2 = argon2::Argon2::default();
    let password = user_data.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(password, &salt)?.to_string();

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
    
    let header_map = http::header::HeaderMap::new();
    header_map.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", token)).map_err(
            |_| AuthError::InternalServer
        )?
    );

    header_map.insert(
        header::HeaderName::from_static("HX-Redirect"),
        header::HeaderValue::from_str("/dashboard").map_err(
            |_| AuthError::InternalServer
        )?
    );

    let response = http::Response::builder()
        .status(StatusCode::OK)
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .header(header::HeaderName::from_static("HX-Redirect"), "/dashboard")
        .body("User registered successfully")
        .map_err(|_| AuthError::InternalServer)?;

    todo!()
}   



#[derive(serde::Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

pub async fn login_user(State(state): State<AppState>, Form(user_data): Form<LoginUser>) -> Result<Json<AuthBody>, AuthError> {
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
        Err(e) => match e {
            argon2::password_hash::Error::Password => return Err(AuthError::WrongPassword),
            _ => return Err(AuthError::PasswordHashing(e)),
        },
        
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_register_user(pool: PgPool) -> Result<(), AuthError> {
        let default_state = AppState::default().await;
        let state = AppState { pool: pool.clone(), smtp_service: default_state.smtp_service.clone() };
        let user_data = RegisterUser {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "Password123!".to_string(),
        };

        let result = register_user(State(state.clone()), Form(user_data)).await;
        assert!(result.is_ok());

        let user = get_user_by_email("test@example.com", &state.pool).await?;
        assert!(user.is_some());

        let user = user.unwrap();
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test User");

        Ok(())
    }

    #[sqlx::test]
    async fn test_login_user(pool: PgPool) -> Result<(), AuthError> {
        let default_state = AppState::default().await;
        let state = AppState { pool: pool.clone(), smtp_service: default_state.smtp_service.clone() };
        
        let user_data = RegisterUser {
            name: "Test User".to_string(),
            email: "test_login@example.com".to_string(),
            password: "Password123!".to_string(),
        };

        let result = register_user(State(state.clone()), Form(user_data)).await;
        assert!(result.is_ok());

        let login_data = LoginUser {
            email: "test_login@example.com".to_string(),
            password: "Password123!".to_string(),
        };

        let result = login_user(State(state.clone()), Form(login_data)).await;
        assert!(result.is_ok());

        Ok(())
    }
}