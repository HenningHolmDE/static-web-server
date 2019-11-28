extern crate envy;
extern crate gotham;
extern crate serde;

#[macro_use]
mod env;

use crate::env::Config;
use gotham::handler::assets::FileOptions;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};

fn default_file_opts(path: &str) -> FileOptions {
    FileOptions::new(path)
        .with_cache_control("no-cache")
        .with_gzip(true)
        .build()
}

pub fn main() {
    let config = envy::prefixed("SERVER_")
        .from_env::<Config>()
        .expect("Unable to parsing config from system env");

    let _addr = format!(
        "{}{}{}",
        config.host.to_string(),
        ":",
        config.port.to_string(),
    );

    let _index = format!("{}/index.html", config.root.to_string());

    let router = build_simple_router(|route| {
        route.get("/").to_file(default_file_opts(&_index));
        route.get("/*").to_dir(default_file_opts(&config.root));
        route
            .get("assets/*")
            .to_dir(default_file_opts(&config.assets));
    });

    gotham::start(_addr, router)
}
