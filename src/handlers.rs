use crate::config::Options;
use gotham::handler::assets::FileOptions;
use gotham::middleware::logger::SimpleLogger;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::builder::*;
use gotham::router::Router;
use log::Level;
use std::path::PathBuf;

pub struct RouterHandler {
    opts: Options,
}

impl RouterHandler {
    /// Create a new instance of `RouterHandler` with given options.
    pub fn new(opts: Options) -> RouterHandler {
        RouterHandler { opts }
    }

    pub fn handle(&self) -> Router {
        let index_file = format!("{}/index.html", &self.opts.root);
        let (chain, pipelines) =
            single_pipeline(new_pipeline().add(SimpleLogger::new(Level::Info)).build());

        // TODO: check if paths exist
        let root_dir = PathBuf::from(&self.opts.root);
        let assets_dir = PathBuf::from(&self.opts.assets);

        build_router(chain, pipelines, |route| {
            route.associate("/", |assoc| {
                assoc.head().to_file(&index_file);
                assoc.get().to_file(&index_file);
            });

            route.associate("/*", |assoc| {
                assoc.head().to_dir(
                    FileOptions::new(&root_dir)
                        .with_cache_control("no-cache")
                        .with_gzip(true)
                        .build(),
                );
                assoc.get().to_dir(
                    FileOptions::new(&root_dir)
                        .with_cache_control("no-cache")
                        .with_gzip(true)
                        .build(),
                );
            });

            if assets_dir.is_absolute() {
                // TODO: use assets basedir as router name
                route.associate("/assets/*", |assoc| {
                    assoc.head().to_dir(
                        FileOptions::new(&assets_dir)
                            .with_cache_control("no-cache")
                            .with_gzip(true)
                            .build(),
                    );
                    assoc.get().to_dir(
                        FileOptions::new(&assets_dir)
                            .with_cache_control("no-cache")
                            .with_gzip(true)
                            .build(),
                    );
                });
            }
        })
    }
}
