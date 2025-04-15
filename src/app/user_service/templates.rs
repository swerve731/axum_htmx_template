use askama::Template;



#[derive(Template)]
#[template(path = "user/dashboard.html")]
pub struct UserDashboardTemplate {
    pub name: String,
    pub email: String,
}