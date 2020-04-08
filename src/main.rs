use gotham::handler::assets::FileOptions;
use gotham::middleware::logger::SimpleLogger;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::builder::*;
use gotham::router::builder::{DefineSingleRoute, DrawRoutes};
use gotham::router::Router;
use log::Level;
use structopt::StructOpt;

mod config;

use crate::config::Options;

fn router(opts: &Options) -> Router {
    let index_file = format!("{}/index.html", opts.root);

    let (chain, pipelines) =
        single_pipeline(new_pipeline().add(SimpleLogger::new(Level::Info)).build());

    build_router(chain, pipelines, |route| {
        route.associate("/", |assoc| {
            assoc.head().to_file(&index_file);
            assoc.get().to_file(index_file);
        });
        route.associate("/*", |assoc| {
            assoc.head().to_dir(
                FileOptions::new(&opts.root)
                    .with_cache_control("no-cache")
                    .with_gzip(true)
                    .build(),
            );
            assoc.get().to_dir(
                FileOptions::new(&opts.root)
                    .with_cache_control("no-cache")
                    .with_gzip(true)
                    .build(),
            );
        });
    })
}

pub fn main() {
    env_logger::init();

    let opts = Options::from_args();
    let addr = format!("{}{}{}", opts.host.to_string(), ":", opts.port.to_string());

    gotham::start(addr, router(&opts))
}
