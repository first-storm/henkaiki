use std::sync::Mutex;

use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, Data, Path},
};
use log::*;

use crate::{
    api::ApiResponse,
    articles::{ArticleId, Articles, Cached},
    cache_recorder::{CacheHit, CacheStats},
};

/// Retrieves a list of all articles without their content.
#[get("/api/v1/articles")]
async fn get_articles(articles_data: Data<Articles>) -> impl Responder {
    match articles_data.get_all_articles_without_content() {
        Ok(articles) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: articles,
            message: None,
        }),
        Err(e) => {
            error!("Error retrieving articles: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: (),
                message: Some("Failed to retrieve articles".into()),
            })
        }
    }
}

/// Retrieves a specific article by ID.
#[get("/api/v1/articles/{id}")]
async fn get_article(
    articles_data: Data<Articles>,
    cache_recorder: Data<Mutex<CacheHit>>,
    path: Path<ArticleId>,
) -> impl Responder {
    let article_id = path.into_inner();
    match articles_data.get_article(article_id) {
        Ok((article, cache)) => {
            // Record cache hit or miss
            {
                let mut recorder = cache_recorder.lock().unwrap();
                if cache == Cached::Yes {
                    recorder.hit();
                } else {
                    recorder.miss();
                }
            }
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: article,
                message: None,
            })
        }
        Err(e) => {
            warn!("Article ID {} not found: {:?}", article_id, e);
            HttpResponse::NotFound().json(ApiResponse::<()> {
                success: false,
                data: (),
                message: Some("Article not found".into()),
            })
        }
    }
}

/// Retrieves articles filtered by a specific tag.
#[get("/api/v1/tags/{tag}/articles")]
async fn get_articles_by_tag(articles_data: Data<Articles>, path: Path<String>) -> impl Responder {
    let tag = path.into_inner();
    match articles_data.get_articles_by_tag(&tag) {
        Ok(articles) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: articles,
            message: None,
        }),
        Err(e) => {
            error!("Error retrieving articles by tag '{}': {:?}", tag, e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: (),
                message: Some("Failed to retrieve articles by tag".into()),
            })
        }
    }
}

/// Refreshes a specific article in the cache.
#[post("/api/v1/admin/articles/{id}/refresh")]
async fn refresh_article_cache(
    articles_data: Data<Articles>,
    path: Path<ArticleId>,
) -> impl Responder {
    let article_id = path.into_inner();
    match articles_data.refresh_article(article_id) {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()> {
            success: true,
            data: (),
            message: Some("Article cache refreshed".into()),
        }),
        Err(e) => {
            error!("Error refreshing article {}: {:?}", article_id, e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: (),
                message: Some("Failed to refresh article cache".into()),
            })
        }
    }
}

/// Clears the entire articles cache.
#[post("/api/v1/admin/cache/clear")]
async fn clear_cache(articles_data: Data<Articles>) -> impl Responder {
    articles_data.clear_cache();
    HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: (),
        message: Some("Cache cleared".into()),
    })
}

/// Refreshes the articles index by reloading from the filesystem.
#[post("/api/v1/admin/index/refresh")]
async fn refresh_index(articles_data: Data<Articles>) -> impl Responder {
    match articles_data.refresh_index() {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()> {
            success: true,
            data: (),
            message: Some("Index refreshed".into()),
        }),
        Err(e) => {
            error!("Error refreshing index: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: (),
                message: Some("Failed to refresh index".into()),
            })
        }
    }
}

/// Retrieves cache statistics.
#[get("/api/v1/admin/cache/stats")]
async fn get_cache_stats(cache_recorder: Data<Mutex<CacheHit>>) -> impl Responder {
    let stats = cache_recorder.lock().unwrap();
    let cache_stats = CacheStats {
        cache_hit: stats.cache_hit,
        cache_miss: stats.cache_miss,
        hit_rate: stats.hit_rate(),
    };
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: cache_stats,
        message: None,
    })
}

/// Resets cache statistics.
#[post("/api/v1/admin/cache/stats/reset")]
async fn reset_cache_stats(cache_recorder: Data<Mutex<CacheHit>>) -> impl Responder {
    cache_recorder.lock().unwrap().reset();
    HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: (),
        message: Some("Cache statistics have been reset".into()),
    })
}

/// Configures the API v1 routes.
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_articles)
        .service(get_article)
        .service(get_articles_by_tag)
        .service(refresh_article_cache)
        .service(clear_cache)
        .service(refresh_index)
        .service(get_cache_stats)
        .service(reset_cache_stats);
}
