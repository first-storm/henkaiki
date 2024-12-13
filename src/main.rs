use actix_web::{App, HttpResponse, HttpServer, get, middleware, web};
use log::*;
use lru::LruCache;
use std::sync::{Arc, Mutex};

mod api;
mod articles;
mod cache_recorder;
mod config;
mod markdown;

use articles::Articles;

use cache_recorder::CacheHit;

/// Health check endpoint to verify that the server is running.
#[get("/health")]
async fn health_check() -> impl actix_web::Responder {
    HttpResponse::Ok().json(api::ApiResponse {
        success: true,
        data: "Server is running",
        message: None,
    })
}

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
            .service(health_check)
            .configure(api::v1::config)
    })
    .bind((config.mainconfig.address.clone(), config.mainconfig.port))?
    .run()
    .await
}
