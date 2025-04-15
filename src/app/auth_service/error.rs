use axum::response::IntoResponse;
use serde::Serialize;


#[derive(derive_more::From, Debug)]
pub enum AuthError {
    UserNotFound,
    EmailAlreadyExists,
    InvalidEmail,
    WrongPassword,
    InvalidPassword {
        has_uppercase: bool,
        has_lowercase: bool,
        has_digit: bool,
        min_length: usize,
        is_long_enough: bool,
    },
    InvalidToken,
    #[from]
    Sqlx(sqlx::Error),

    #[from]
    Jwt(jsonwebtoken::errors::Error),


}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("AuthError: {:?}", self);

        let (status, body) = match self {
            AuthError::WrongPassword => {
                let status = axum::http::StatusCode::UNAUTHORIZED;
                let body = "Wrong password".to_string();
                (status, body)
            },
            AuthError::UserNotFound => {
                let status = axum::http::StatusCode::NOT_FOUND;
                let body = "User not found".to_string();
                (status, body)
            },
            AuthError::EmailAlreadyExists => {
                let status = axum::http::StatusCode::CONFLICT;
                let body = "Email already exists".to_string();
                (status, body)
            },
            AuthError::InvalidEmail => {
                let status = axum::http::StatusCode::BAD_REQUEST;
                let body = "Invalid email".to_string();
                (status, body)
            },
            AuthError::InvalidPassword{
                has_uppercase,
                has_lowercase,
                has_digit,
                min_length,
                is_long_enough,
            } => {
                let status = axum::http::StatusCode::BAD_REQUEST;
                // return json of error 
                let body = serde_json::json!({
                    "error": "Invalid password",
                    "has_uppercase": has_uppercase,
                    "has_lowercase": has_lowercase,
                    "has_digit": has_digit,
                    "min_length": min_length,
                    "is_long_enough": is_long_enough,
                });
                (status, body.to_string())
            },
            AuthError::Sqlx(err) => {
                let status = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                let body = format!("Database error: {:?}", err);
                (status, body)
            },
            AuthError::Jwt(err) => {
                let status = axum::http::StatusCode::UNAUTHORIZED;
                let body = format!("JWT error: {:?}", err);
                (status, body)
            },
            AuthError::InvalidToken => {
                let status = axum::http::StatusCode::UNAUTHORIZED;
                let body = "Invalid token try signing back in".to_string();
                (status, body)
            }
        };

        (status, body).into_response()
    }
}

