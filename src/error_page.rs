use gotham::router::response::extender::ResponseExtender;
use gotham::state::State;
use hyper::{Body, Response};
use std::fs;
use std::path::Path;

static PAGE_404: &str = "<h2>404</h2><p>Content could not found</p>";
static PAGE_50X: &str =
    "<h2>50x</h2><p>Service is temporarily unavailable due an unexpected error</p>";

/// Gotham extender for custom HTTP status 404 error page.
pub struct ErrorPage404 {
    page_content: String,
}

/// Gotham extender for custom HTTP status 50x error page.
pub struct ErrorPage50x {
    page_content: String,
}

impl ErrorPage404 {
    /// Create a new instance of `ErrorPage404` Gotham extender with a given html pages.
    pub fn new<P: AsRef<Path>>(page_path: P) -> ErrorPage404 {
        let page_content = if Path::new(&page_path.as_ref()).exists() {
            fs::read_to_string(page_path).unwrap()
        } else {
            String::from(PAGE_404)
        };
        ErrorPage404 { page_content }
    }
}

impl ErrorPage50x {
    /// Create a new instance of `ErrorPage50x` Gotham extender with a given html page.
    pub fn new<P: AsRef<Path>>(page_path: P) -> ErrorPage50x {
        let page_content = if Path::new(&page_path.as_ref()).exists() {
            fs::read_to_string(page_path).unwrap()
        } else {
            String::from(PAGE_50X)
        };

        ErrorPage50x { page_content }
    }
}

impl ResponseExtender<Body> for ErrorPage404 {
    fn extend(&self, _state: &mut State, response: &mut Response<Body>) {
        *response.body_mut() = Body::from(self.page_content.to_string());
    }
}

impl ResponseExtender<Body> for ErrorPage50x {
    fn extend(&self, _state: &mut State, response: &mut Response<Body>) {
        *response.body_mut() = Body::from(self.page_content.to_string());
    }
}
