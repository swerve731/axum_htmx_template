use askama::Template;


#[derive(Template, askama_derive_axum::IntoResponse)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate {}



#[derive(Template, askama_derive_axum::IntoResponse)]
#[template(path = "auth/login.html")]
pub struct RegisterTemplate {}


