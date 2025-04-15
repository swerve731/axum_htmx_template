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


