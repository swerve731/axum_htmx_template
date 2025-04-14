use axum::response::IntoResponse;


#[derive(derive_more::From, Debug)]
pub enum AuthError {
    WrongPassword,
    UserNotFound,
    EmailAlreadyExists,
    InvalidEmail,
    InvalidPassword,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
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
            AuthError::InvalidPassword => {
                let status = axum::http::StatusCode::BAD_REQUEST;
                let body = "Invalid password".to_string();
                (status, body)
            },
        };

        (status, body).into_response()
    }
}

