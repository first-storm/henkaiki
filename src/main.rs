use actix_web::{middleware, web, App, HttpServer};
use log::*;
use lru::LruCache;
use std::sync::{Arc, Mutex};

mod api;
mod articles;
mod config;
mod cache_recorder;
mod markdown;

use articles::Articles;

use cache_recorder::CacheHit;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();

    // Load configuration
    let config = &*config::CONFIG;
    info!("Config loaded: {:?}", config);

    // Initialize the shared LruCache for articles
    let cache = Arc::new(Mutex::new(LruCache::new(
        config.mainconfig.max_cached_articles,
    )));

    // Create the shared Articles instance
    let articles_instance = Articles::new(
        config.mainconfig.articles_dir.clone().into(),
        Arc::clone(&cache),
    );

    // Construct shared cache recorder
    let cache_recorder = web::Data::new(Mutex::new(CacheHit::new()));

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(articles_instance.clone()))
            .app_data(cache_recorder.clone())
            .configure(api::v1::config)
    })
    .bind((
        config.mainconfig.address.clone(),
        config.mainconfig.port,
    ))?
    .run()
    .await
}
