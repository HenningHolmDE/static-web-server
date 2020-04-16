use gotham::handler::assets::FileOptions;
use gotham::middleware::logger::SimpleLogger;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::builder::*;
use gotham::router::Router;
use log::Level;

use crate::config::Options;
use crate::helpers;
use crate::logger;

pub struct RouterHandler {
    opts: Options,
}

impl RouterHandler {
    /// Create a new instance of `RouterHandler` with given options.
    pub fn new(opts: Options) -> RouterHandler {
        RouterHandler { opts }
    }

    /// Handle the server router configuration
    pub fn handle(&self) -> Router {
        // Setup logging
        let (chain, pipelines) =
            single_pipeline(new_pipeline().add(SimpleLogger::new(Level::Info)).build());

        // Options definition
        let opts = &self.opts;

        // Check the root directory
        let root_dir = match helpers::validate_dirpath(&opts.root) {
            Err(err) => {
                error!("{}", helpers::path_error_fmt(err, "root", &opts.root));
                std::process::exit(1);
            }
            Ok(val) => val,
        };
        // Check the assets directory
        let assets_dir = match helpers::validate_dirpath(&opts.assets) {
            Err(err) => {
                error!("{}", helpers::path_error_fmt(err, "assets", &opts.assets));
                std::process::exit(1);
            }
            Ok(val) => val,
        };

        // Default index html file
        let index_file = format!("{}/index.html", &root_dir.display());

        // Routes configuration (GET & HEAD)
        build_router(chain, pipelines, |route| {
            // Root route configuration
            route.get_or_head("/").to_file(&index_file);

            // Root wilcard configuration
            route.get_or_head("/*").to_dir(
                FileOptions::new(&root_dir)
                    .with_cache_control("no-cache")
                    .with_gzip(true)
                    .build(),
            );

            // route.add_response_extender(StatusCode::NOT_FOUND, ErrorExtender);

            let assets_dirname = match assets_dir.iter().last() {
                None => {
                    error!("assets directory name was not determined");
                    std::process::exit(1);
                }
                Some(val) => val.to_str().unwrap().to_string(),
            };

            // Use assets base directory name as route endpoint
            let assets_route = &format!("/{}/*", assets_dirname);

            route.get_or_head("/").to_dir(
                FileOptions::new(&assets_dir)
                    .with_cache_control("no-cache")
                    .with_gzip(true)
                    .build(),
            );

            let listen = format!("{}{}{}", &opts.host, ":", &opts.port);
            let proto = if opts.tls { "HTTPS" } else { "HTTP" };

            // Server info
            logger::log_server(&format!(
                "Static {} Server \"{}\" is listening on {}",
                proto, opts.name, listen
            ));
            logger::log_server("Root endpoint   -> HEAD,GET /");
            logger::log_server(&format!("Assets endpoint -> HEAD,GET {}", &assets_route));
        })
    }
}
