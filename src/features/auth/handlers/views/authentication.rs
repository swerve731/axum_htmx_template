use askama::Template;






#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate;

#[derive(Template)]
#[template(path = "auth/register.html")]
pub struct RegisterTemplate;


