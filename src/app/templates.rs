use askama::Template;

#[derive(Template, askama_derive_axum::IntoResponse)]
#[template(path = "index.html")]
pub struct IndexTemplate{}