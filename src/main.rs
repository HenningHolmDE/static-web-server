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
