
use askama::Template;
use axum::{response::Html, routing::{get, post}, Router};
use super::WebService;

use crate::features::auth::handlers::*;
pub struct AuthService{}

impl WebService for AuthService {
    fn view_router(state: super::server::ServerState) -> axum::Router<super::server::ServerState> {
        Router::new()

            .route("/login", get(|| async {
                Html(views::authentication::LoginTemplate{}.render().map_err(
                   |e| crate::ServerError::Askama(e)
                ))
            }))
            .route("/register", get(|| async {
                Html(views::authentication::RegisterTemplate{}.render().map_err(
                   |e| crate::ServerError::Askama(e)
                ))
            }))
            
            //this directs the user to email login -> enter code -> change password -> login
            .route("/reset-password", get(|| async {
                Html(views::password_reset::EmailForm{}.render().map_err(
                   |e| crate::ServerError::Askama(e)
                ))
            }))            
      
            .with_state(state)
    }

    fn api_router(state: super::server::ServerState) -> axum::Router<super::server::ServerState> {
        Router::new()
            .route("/logout", post(api::authentication::logout))
            .route("/login", post(api::authentication::login))
            .route("/register", post(api::authentication::register))
            .route("/email-code", post(api::password_reset::email_code))
            .route("/code-login", post(api::password_reset::code_login))
            .route("/change-password", post(api::password_reset::change_password))
            .with_state(state)
    }
}