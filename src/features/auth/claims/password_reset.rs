use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{header, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::Cookie;
use jsonwebtoken::Validation;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{error::ClaimsError, Claims, KEYS}; // Adjusted imports
use crate::utils::extract_cookie_value; // Adjusted imports
#[derive(Serialize, Deserialize, Debug, Clone)] // Added Clone
pub struct PasswordResetClaim {
    pub exp: usize,      
    pub code: String,   
    pub authorized: bool,
    pub email: String
}

impl PasswordResetClaim {
    // Keep specific constants here
    pub const EXPIRE_TIME_MINUTES: i64 = 15;
    pub const CODE_LENGTH: usize = 6;

    pub fn new(email: String) -> Self {
        // Use thread_rng for convenience
        let rng = rand::rng();
        let code: String  = rng.sample_iter(rand::distr::Alphanumeric).take(Self::CODE_LENGTH).map(|v|v.to_ascii_uppercase() as char).collect();

        let expiration_time =
            chrono::Utc::now() + chrono::Duration::minutes(Self::EXPIRE_TIME_MINUTES);

        Self {
            email,
            exp: expiration_time.timestamp() as usize, // Cast to usize
            code,
            authorized: false,
        }
    }

    // Method to check if the provided code matches
    pub fn authorize(&mut self, submitted_code: &str) -> bool {

        if self.code == submitted_code.to_ascii_uppercase() {
            self.authorized = true;
            true
        } else {
            false
        }
    }
}

// Implement the main Claims trait
impl Claims for PasswordResetClaim {
    const EXP_TIME_HOURS: i64 = 0;
    const SUCCESS_REDIRECT_URI: &'static str = "/dashboard"; 
    const COOKIE_NAME: &'static str = "token";

    fn cookie(&self) -> Result<String, ClaimsError> {
        let token = self.token()?;
        let cookie = Cookie::build((Self::COOKIE_NAME, token)) // Use specific cookie name
            .path("/") // Or maybe a more specific path like "/auth"?
            .http_only(true)
            .secure(true) // Recommended for sensitive cookies
            .to_string();

        Ok(cookie)
    }

    fn token(&self) -> Result<String, ClaimsError> {
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), self, &KEYS.encoding)?;
        Ok(token)
    }

    fn from_token(token: String) -> Result<Self, ClaimsError>
    where
        Self: Sized
    {
        let token_data = jsonwebtoken::decode(token.as_str(), &KEYS.decoding, &Validation::default())
                    .map_err(|e| {
                        tracing::debug!("Error decoding token: {:?}", e);
                        ClaimsError::InvalidToken
                    })?;

        Ok(token_data.claims)
    }
}

impl<S> FromRequestParts<S> for PasswordResetClaim
where
    S: Send + Sync,
{
    type Rejection = ClaimsError; // Use ClaimsError

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookies_header = parts
            .headers
            .get(header::COOKIE)
            .ok_or(ClaimsError::TokenNotFound)?;

        let cookies_str = cookies_header
            .to_str()
            .map_err(|_| ClaimsError::InvalidToken)?; // Header value is not UTF-8

        // Use the helper function with the specific cookie name
        let token = extract_cookie_value(cookies_str, Self::COOKIE_NAME)
            .ok_or(ClaimsError::TokenNotFound)?; // Specific cookie not found

        tracing::debug!("Email Auth Token found: {}", token);

        // Use the static from_token method for decoding and validation
        let claims = Self::from_token(token)?;

        Ok(claims)
    }
}

// Define how returning this claim results in a response
impl IntoResponse for PasswordResetClaim {
    fn into_response(self) -> Response {
        let cookie = match self.cookie() {
            Ok(cookie) => cookie,
            Err(err) => return err.into_response()
        };

        Response::builder()
            // .header(HxRedirect::HEADER_NAME, Self::SUCCESS_REDIRECT_URI)
            .header(header::SET_COOKIE, cookie) // Use the constant for clarity
            .body(Body::empty())
            .unwrap()
    }
}
