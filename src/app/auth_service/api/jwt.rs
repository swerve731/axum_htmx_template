use std::sync::LazyLock;

use axum::{
    body::Body, extract::FromRequestParts, http::{header, request::Parts, StatusCode, Uri}, response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::Cookie;
use jsonwebtoken::{EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::app::{auth_service::error::AuthError, utils::HxRedirect};

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize,
}

impl Claims {
    pub const EXP_TIME_HOURS: i64 = 168;

    pub fn new(user_id: String) -> Claims{
        let expiration_time = chrono::Utc::now() + chrono::Duration::hours(Self::EXP_TIME_HOURS);

        Self {
            user_id,
            exp: expiration_time.timestamp() as usize,
        }
    }
}

// Extract JWT from cookie
impl<S> FromRequestParts<S> for Claims
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

// New AuthResponse: sets JWT as HttpOnly cookie and redirects
pub struct AuthResponse {
    pub token: String,
    pub redirect_uri: Uri,
}

impl IntoResponse for AuthResponse {
    fn into_response(self) -> Response {
        let cookie = Cookie::build(("token", self.token))
            .path("/")
            .http_only(true)
            .to_string();
        
        Response::builder()
            .header(HxRedirect::HEADER_NAME, self.redirect_uri.to_string())
            .header(header::SET_COOKIE, cookie) // Use the constant for clarity
            .body(Body::empty())
            .unwrap()

        // HxRedirect::to(self.redirect_uri.to_string())
        //     .response_builder()
        //     .header(header::SET_COOKIE, cookie)
        //     .body(Body::empty())
        //     .unwrap_or((StatusCode::INTERNAL_SERVER_ERROR, "could not build response").into_response())

    }
}
