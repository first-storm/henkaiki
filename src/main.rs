use actix_web::{middleware, web, App, HttpServer};
use log::*;
use lru::LruCache;
use std::sync::{Arc, Mutex};

mod api;
mod articles;
mod config;
mod markdown;

use articles::Articles;

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

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            // Add middleware
            .wrap(middleware::Logger::default())
            // Share the Articles instance with handlers
            .app_data(web::Data::new(articles_instance.clone()))
            // Define routes
            .configure(api::v1::config) // Configure API routes
    })
    .bind((
        config.mainconfig.address.clone(),
        config.mainconfig.port,
    ))?
    .run()
    .await
}
