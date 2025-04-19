
use axum::response::IntoResponse;

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
    NoToken,
    InvalidToken,
    FailedToCreateEmailToken,
    PasswordsDontMatch,

    #[from]
    Sqlx(sqlx::Error),

    #[from]
    Jwt(jsonwebtoken::errors::Error),

    #[from]
    PasswordHashing(argon2::password_hash::Error),

    #[from]
    Axum(axum::Error),

    #[from]
    Http(http::Error),

    InternalServer,

    #[from]
    Smtp(crate::app::smtp_service::error::SmtpError),

    #[from]
    Askama(askama::Error),
    
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
                return axum::response::Redirect::to("/auth/login").into_response();
            },
            AuthError::PasswordHashing(err) => {
            let status = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            tracing::error!("URGENT: Password hashing error: {:?}", err);
            let body = format!("Unknown Server Error");
            (status, body)
            },
            AuthError::Axum(err) => {
                let status = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                let body = format!("Axum error: {:?}", err);
                (status, body)
            },
            AuthError::Http(err) => {
                let status = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                let body = format!("HTTP error: {:?}", err);
                (status, body)
            },
            AuthError::InternalServer => {
                let status = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                tracing::error!("URGENT: Internal server error");
                let body = "Internal server error".to_string();
                (status, body)
            },
            AuthError::NoToken => {
                return axum::response::Redirect::to("/auth/login").into_response();
            },
            AuthError::FailedToCreateEmailToken => {
                let status = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                tracing::error!("URGENT: Failed to create email token");
                let body = "Failed to create email token".to_string();

                (status, body)
            },
            AuthError::Smtp(err) => {
                let status = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                tracing::error!("URGENT: SMTP error: {:?}", err);
                let body = format!("Emailing error");
                (status, body)
            },
            AuthError::Askama(err) => {
                let status = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                tracing::error!("URGENT: Askama error: {:?}", err);
                let body = format!("Rendering error");
                (status, body)
            },
            AuthError::PasswordsDontMatch => {
                let status = axum::http::StatusCode::BAD_REQUEST;
                tracing::debug!("Passwords dont match");
                let body = format!("Passwords dont match");
                (status, body)
            }
        };

        (status, body).into_response()
    }
}

