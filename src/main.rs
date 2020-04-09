use structopt::StructOpt;

mod config;
mod handlers;

use crate::config::Options;
use crate::handlers::RouterHandler;

pub fn main() {
    env_logger::init();

    let opts = Options::from_args();
    let addr = format!("{}{}{}", opts.host.to_string(), ":", opts.port.to_string());
    let router = RouterHandler::new(opts);

    gotham::start(addr, router.handle())
}
