use argon2::password_hash::ParamsString;
use askama::Template;


#[derive(Template, askama_derive_axum::IntoResponse)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate {}



#[derive(Template, askama_derive_axum::IntoResponse)]
#[template(path = "auth/register.html")]
pub struct RegisterTemplate {}

#[derive(Template, askama_derive_axum::IntoResponse)]
#[template(path = "auth/reset_password.html")]
pub struct ResetPasswordTemplate {}

#[derive(Template, askama_derive_axum::IntoResponse)]
#[template(path = "auth/email_login.html")]
pub struct EmailLoginTemplate {}

#[derive(Template)]
#[template(path = "auth/email_code.html")]
pub struct EmailCodeTemplate {
    pub email: String,
    pub expire_time: String
}

#[derive(Template, askama_derive_axum::IntoResponse)]
#[template(path = "auth/change_password.html")]
pub struct ChangePasswordTemplate {}
