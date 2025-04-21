use askama::DynTemplate;
use axum::{body::Body, response::{Html, IntoResponse}};
use http::Response;


use axum_extra::extract::cookie::Cookie;

use crate::ServerError;



pub async fn template_response(template: Box<dyn DynTemplate>) -> impl IntoResponse {
    Html(
        template
            .dyn_render()
            .map_err(|e| ServerError::Askama(e))
        ).into_response()
}


pub fn extract_cookie_value(cookies_str: &str, cookie_name: &str) -> Option<String> {
    let cookies = Cookie::split_parse(cookies_str);
    let token = cookies
        .into_iter()
        .find(|c| {
            c.as_ref().map(|c| c.name() == cookie_name).unwrap_or(false)
        })
        .map(|c| c.map(|c| c.value().to_string()))
        .transpose()
        .ok()?;
        
    token

}

pub struct HxRedirect {
    to: String
}

impl HxRedirect {
    pub fn to(to: String) -> HxRedirect {
        HxRedirect { to }
    }
}

impl HxRedirect {
    pub const HEADER_NAME: &str = "HX-Redirect";

    pub fn response_builder(self) -> http::response::Builder {
        Response::builder()
            // .status(303)
            .header("HX-Redirect", self.to.clone())
            // .header("Location",self.to.clone())
    }
}
impl IntoResponse for HxRedirect {
    fn into_response(self) -> axum::response::Response {
        Response::builder()
            // .status(303)
            .header("HX-Redirect", self.to.clone())
            // .header("Location", self.to.clone())
            .body(Body::empty())
            .unwrap()
    }
}