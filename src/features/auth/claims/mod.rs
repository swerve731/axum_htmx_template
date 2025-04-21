pub mod error;
pub mod authorization;
pub mod password_reset;

use std::sync::LazyLock;

use axum::response::IntoResponse;
use error::ClaimsError;
use jsonwebtoken::{DecodingKey, EncodingKey};


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

pub trait Claims: IntoResponse {
    const EXP_TIME_HOURS: i64;
    const SUCCESS_REDIRECT_URI: &'static str;
    const COOKIE_NAME: &'static str;

    fn cookie(&self) -> Result<String, ClaimsError>;

    fn token(&self) -> Result<String, ClaimsError>;

    fn from_token(token: String) -> Result<Self, ClaimsError> where Self: Sized;
}

