use axum::{body::Body, response::IntoResponse};
use http::{HeaderMap, HeaderName, HeaderValue, Response};



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