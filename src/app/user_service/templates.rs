use askama::Template;



#[derive(Template)]
#[template(path = "user/dashboard.html")]
pub struct UserDashboard {
    pub name: String,
    pub email: String,
}