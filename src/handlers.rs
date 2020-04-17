use gotham::handler::assets::FileOptions;
use gotham::middleware::logger::SimpleLogger;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::set::{finalize_pipeline_set, new_pipeline_set};
use gotham::router::builder::*;
use gotham::router::Router;
use hyper::StatusCode;
use log::Level;

use crate::config::Options;
use crate::error_page::{ErrorPage404, ErrorPage50x};
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
        let pipelines = new_pipeline_set();

        // Setup logging middleware
        let (pipelines, default) =
            pipelines.add(new_pipeline().add(SimpleLogger::new(Level::Info)).build());

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

        // Setup custom error pages middleware
        // let (pipelines, extended) = pipelines.add(
        //     new_pipeline()
        //         .add(ErrorPageMiddleware::new(&opts.page404, &opts.page50x))
        //         .build(),
        // );

        // Setup middleware pipelines
        let pipeline_set = finalize_pipeline_set(pipelines);
        let default_chain = (default, ());
        // let extended_chain = (extended, default_chain);

        // Routes configuration (GET & HEAD)
        build_router(default_chain, pipeline_set, |route| {
            // Custom error pages based on HTTP status
            // HTTP status 404
            route.add_response_extender(StatusCode::NOT_FOUND, ErrorPage404::new(&opts.page404));
            // HTTP status 50x
            route.add_response_extender(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorPage50x::new(&opts.page50x),
            );
            route.add_response_extender(StatusCode::BAD_GATEWAY, ErrorPage50x::new(&opts.page50x));
            route.add_response_extender(
                StatusCode::SERVICE_UNAVAILABLE,
                ErrorPage50x::new(&opts.page50x),
            );
            route.add_response_extender(
                StatusCode::GATEWAY_TIMEOUT,
                ErrorPage50x::new(&opts.page50x),
            );

            // Root route configuration
            route.get_or_head("/").to_file(&index_file);

            // Root wilcard configuration
            route.get_or_head("/*").to_dir(
                FileOptions::new(&root_dir)
                    .with_cache_control("no-cache")
                    .with_gzip(true)
                    .build(),
            );

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
