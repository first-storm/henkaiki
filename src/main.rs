// main.rs
mod articles;
mod config;
mod markdown;

use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, put, web};
use articles::Articles;
use log::*;
use lru::LruCache;
use std::sync::{Arc, Mutex};

type ArticleId = i32;

#[get("/{id}")]
async fn get_article_handler(
    articles_data: web::Data<Articles>,
    path: web::Path<ArticleId>,
) -> impl Responder {
    let article_id = path.into_inner();
    match articles_data.get_article(article_id) {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(e) => {
            error!("Error retrieving article {}: {:?}", article_id, e);
            HttpResponse::NotFound().body("Article not found")
        }
    }
}

#[get("/all")]
async fn get_all_articles_handler(articles_data: web::Data<Articles>) -> impl Responder {
    match articles_data.get_all_articles_without_content() {
        Ok(summaries) => HttpResponse::Ok().json(summaries),
        Err(e) => {
            error!("Error retrieving article summaries: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve articles")
        }
    }
}

#[delete("/cache")]
async fn clear_cache_handler(articles_data: web::Data<Articles>) -> impl Responder {
    articles_data.clear_cache();
    HttpResponse::Ok().body("Cache cleared")
}

#[put("/{id}/cache")]
async fn refresh_article_handler(
    articles_data: web::Data<Articles>,
    path: web::Path<ArticleId>,
) -> impl Responder {
    let article_id = path.into_inner();
    match articles_data.refresh_article(article_id) {
        Ok(_) => HttpResponse::Ok().body("Article cache refreshed"),
        Err(e) => {
            error!("Error refreshing article {}: {:?}", article_id, e);
            HttpResponse::InternalServerError().body("Failed to refresh article cache")
        }
    }
}

/// Handler to refresh the article index.
#[put("")]
async fn refresh_index_handler(articles_data: web::Data<Articles>) -> impl Responder {
    match articles_data.refresh_index() {
        Ok(_) => HttpResponse::Ok().body("Index refreshed"),
        Err(e) => {
            error!("Error refreshing index: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to refresh index")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Initialize the logger

    let config = &*config::CONFIG;
    info!("Config loaded: {:?}", config);

    // Initialize the shared LruCache
    let cache = Arc::new(Mutex::new(LruCache::new(
        config.mainconfig.max_cached_articles,
    )));

    // Create an Articles instance with the shared cache
    let articles_instance = Articles::new(
        config.mainconfig.articles_dir.clone().into(),
        Arc::clone(&cache),
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(articles_instance.clone()))
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/article")
                            .service(clear_cache_handler)
                            .service(get_all_articles_handler)
                            .service(get_article_handler)
                            .service(refresh_article_handler),
                    )
                    .service(
                        web::scope("/index")
                            .service(refresh_index_handler),
                    )
            )
    })
    .bind(("127.0.0.1", 8080))? // Bind the server to the address
    .run()
    .await
}
