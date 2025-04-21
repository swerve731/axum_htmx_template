use askama::Template;





#[derive(Template)]
#[template(path = "auth/reset_password.html")]
pub struct EmailForm {}

#[derive(Template)]
#[template(path = "auth/fragments/forms/password_reset/code_login.html")]
pub struct CodeForm {
    pub email: String,
    pub expire_time: String
}

#[derive(Template)]
#[template(path = "auth/fragments/forms/password_reset/change_password.html")]
pub struct ResetPasswordForm{}


