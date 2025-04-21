use axum::{
    body::Body, extract::FromRequestParts, http::{header, request::Parts}, response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::Cookie;
use uuid::Uuid;
use super::{error::ClaimsError, Claims};
use jsonwebtoken::Validation;
use serde::{Deserialize, Serialize};

use crate::{features::auth::claims::KEYS, utils::HxRedirect};




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizationClaim {
    pub user_id: uuid::Uuid,
    pub exp: usize,
    pub is_main_claims: bool //this is so it does not grab the EmailLoginAuthorizationClaim

}


impl Claims for AuthorizationClaim 
{
    const EXP_TIME_HOURS: i64 = 168;
    const SUCCESS_REDIRECT_URI: &'static str = "/dashboard";
    const COOKIE_NAME: &'static str = "token";

    fn cookie(&self) ->  Result<String, ClaimsError> {
        let cookie = Cookie::build((Self::COOKIE_NAME, self.token()?.as_str()))
            .path("/")
            .http_only(true)
            .to_string();
        Ok(cookie)
    }

    fn token(&self) -> Result<String, ClaimsError> {
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), self, &KEYS.encoding)?;
        Ok(token)
    }

    fn from_token(token: String) -> Result<Self, ClaimsError> where Self: Sized {
        let token_data = jsonwebtoken::decode(token.as_str(), &KEYS.decoding, &Validation::default())
        .map_err(|e| {
            tracing::debug!("Error decoding token: {:?}", e);
            ClaimsError::InvalidToken
        })?;

        return Ok(token_data.claims)    
    }
}

impl AuthorizationClaim {

    pub fn new(user_id: Uuid) -> AuthorizationClaim{
        let expiration_time = chrono::Utc::now() + chrono::Duration::hours(Self::EXP_TIME_HOURS);

        Self {
            user_id,
            exp: expiration_time.timestamp() as usize,
            is_main_claims: true
        }
    }
}

// Extract JWT from cookie
impl<S> FromRequestParts<S> for AuthorizationClaim
where
    S: Send + Sync,
{
    type Rejection = ClaimsError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookies= parts.headers.get(header::COOKIE);
        
        match cookies {
            Some(cookies) => {
                // parse the cookies and get the token
                let cookies_str = cookies.to_str().map_err(|_| ClaimsError::InvalidToken)?;
                
                let token= crate::utils::extract_cookie_value(cookies_str, Self::COOKIE_NAME);

                if token.is_none() {
                    return Err(ClaimsError::TokenNotFound)
                }
                let token = token.unwrap();

                tracing::debug!("Token: {:?}", token);

                let token_data = jsonwebtoken::decode(token.as_str(), &KEYS.decoding, &Validation::default())
                    .map_err(|e| {
                        tracing::debug!("Error decoding token: {:?}", e);
                        ClaimsError::InvalidToken
                    })?;
                return Ok(token_data.claims)

            },
            None => return Err(ClaimsError::TokenNotFound )
        }

    }
}


impl IntoResponse for AuthorizationClaim {
    fn into_response(self) -> Response {
        let cookie = match self.cookie() {
            Ok(cookie) => cookie,
            Err(err) => return err.into_response()
        };

        Response::builder()
            .header(HxRedirect::HEADER_NAME, Self::SUCCESS_REDIRECT_URI)
            .header(header::SET_COOKIE, cookie) // Use the constant for clarity
            .body(Body::empty())
            .unwrap()
    }
}

