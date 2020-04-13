#[macro_use]
extern crate log;

use structopt::StructOpt;

mod config;
mod handlers;
mod helpers;
mod logger;

use crate::config::Options;
use crate::handlers::RouterHandler;

pub fn main() {
    let opts = Options::from_args();

    logger::init(&opts.log_level);

    let addr = format!("{}{}{}", &opts.host, ":", &opts.port);
    let router = RouterHandler::new(opts);

    gotham::start(addr, router.handle())
}

#[cfg(test)]
mod tests {
    extern crate http;

    use super::*;
    use gotham::test::TestServer;
    use http::StatusCode;

    #[test]
    fn receive_router_response() {
        let opts = Options::from_args();
        let router = RouterHandler::new(opts);
        let test_server = TestServer::new(router.handle()).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn receive_404_response() {
        let opts = Options::from_args();
        let router = RouterHandler::new(opts);
        let test_server = TestServer::new(router.handle()).unwrap();
        let response = test_server
            .client()
            .get("http://localhost/dummy")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
