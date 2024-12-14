use actix_web::{
    delete, get, post,
    web::{self, Data, Path, Query},
    HttpResponse, Responder,
};
use log::*;
use serde::Deserialize;
use std::sync::Mutex;

use crate::{
    api::ApiResponse,
    articles::{ArticleId, Articles, CachedStatus},
    cache_recorder::{CacheHit, CacheStats},
};

const DEFAULT_PAGE_SIZE: usize = 10;

#[derive(Deserialize)]
struct PaginationParams {
    limit: Option<usize>,
    page: Option<usize>,
}

/// Retrieves a list of articles with optional pagination
#[get("/api/v1/articles")]
async fn list_articles(
    articles_data: Data<Articles>,
    query: Query<PaginationParams>,
) -> impl Responder {
    // If both limit and page are provided, use pagination
    if let (Some(limit), Some(page)) = (query.limit, query.page) {
        match articles_data.list_article_summaries_paginated(limit, page) {
            Ok(articles) => HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: articles,
                message: None,
            }),
            Err(e) => {
                error!("Error retrieving paginated articles: {:?}", e);
                HttpResponse::BadRequest().json(ApiResponse::<()> {
                    success: false,
                    data: (),
                    message: Some("Invalid pagination parameters".into()),
                })
            }
        }
    } else {
        // If no pagination parameters, return all articles
        match articles_data.list_article_summaries() {
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
}

/// Get total number of pages for articles
#[get("/api/v1/articles/pages")]
async fn get_article_pages(
    articles_data: Data<Articles>,
    query: Query<PaginationParams>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(DEFAULT_PAGE_SIZE);
    let pages = articles_data.get_article_summary_page_count(limit);
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: pages,
        message: None,
    })
}

/// Retrieves a specific article by ID
#[get("/api/v1/articles/{id}")]
async fn get_article(
    articles_data: Data<Articles>,
    cache_recorder: Data<Mutex<CacheHit>>,
    path: Path<ArticleId>,
) -> impl Responder {
    let article_id = path.into_inner();
    match articles_data.get_article(article_id) {
        Ok((article, cache_status)) => {
            // Record cache hit or miss
            {
                let mut recorder = cache_recorder.lock().unwrap();
                match cache_status {
                    CachedStatus::Cached => recorder.hit(),
                    CachedStatus::NotCached => recorder.miss(),
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

/// Refreshes the articles index
#[post("/api/v1/articles/index/refresh")]
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

/// Clears the articles cache
#[delete("/api/v1/articles/cache")]
async fn clear_cache(articles_data: Data<Articles>) -> impl Responder {
    articles_data.clear_cache();
    HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: (),
        message: Some("Cache cleared".into()),
    })
}

/// Refreshes a specific article in the cache
#[post("/api/v1/articles/{id}/refresh")]
async fn refresh_article(
    articles_data: Data<Articles>,
    path: Path<ArticleId>,
) -> impl Responder {
    let article_id = path.into_inner();
    match articles_data.refresh_article(article_id) {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()> {
            success: true,
            data: (),
            message: Some("Article refreshed".into()),
        }),
        Err(e) => {
            error!("Error refreshing article {}: {:?}", article_id, e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: (),
                message: Some("Failed to refresh article".into()),
            })
        }
    }
}

/// Retrieves articles by tag with optional pagination
#[get("/api/v1/articles/tags/{tag}")]
async fn list_articles_by_tag(
    articles_data: Data<Articles>,
    path: Path<String>,
    query: Query<PaginationParams>,
) -> impl Responder {
    let tag = path.into_inner();
    
    // If both limit and page are provided, use pagination
    if let (Some(limit), Some(page)) = (query.limit, query.page) {
        match articles_data.list_article_summaries_by_tag_paginated(&tag, limit, page) {
            Ok(articles) => HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: articles,
                message: None,
            }),
            Err(e) => {
                error!("Error retrieving paginated articles by tag '{}': {:?}", tag, e);
                HttpResponse::BadRequest().json(ApiResponse::<()> {
                    success: false,
                    data: (),
                    message: Some("Invalid pagination parameters".into()),
                })
            }
        }
    } else {
        // If no pagination parameters, return all articles with the tag
        match articles_data.list_article_summaries_by_tag(&tag) {
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
}

/// Get total number of pages for articles with a specific tag
#[get("/api/v1/articles/tags/{tag}/pages")]
async fn get_tag_pages(
    articles_data: Data<Articles>,
    path: Path<String>,
    query: Query<PaginationParams>,
) -> impl Responder {
    let tag = path.into_inner();
    let limit = query.limit.unwrap_or(DEFAULT_PAGE_SIZE);
    let pages = articles_data.get_article_summary_by_tag_page_count(&tag, limit);
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: pages,
        message: None,
    })
}

/// Retrieves cache statistics
#[get("/api/v1/articles/cache/stats")]
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

/// Resets cache statistics
#[post("/api/v1/articles/cache/stats/reset")]
async fn reset_cache_stats(cache_recorder: Data<Mutex<CacheHit>>) -> impl Responder {
    cache_recorder.lock().unwrap().reset();
    HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: (),
        message: Some("Cache statistics have been reset".into()),
    })
}

/// Configures the API v1 routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_articles)
        .service(get_article_pages)
        .service(get_article)
        .service(refresh_index)
        .service(clear_cache)
        .service(refresh_article)
        .service(list_articles_by_tag)
        .service(get_tag_pages)
        .service(get_cache_stats)
        .service(reset_cache_stats);
}