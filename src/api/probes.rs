use std::sync::Arc;

use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use tokio::sync::Mutex;

use crate::{settings::get_settings, storage::cache::CacheClient};

#[get("/probes/health/")]
async fn health(cache: web::Data<Arc<Mutex<CacheClient>>>) -> impl Responder {
    let settings = get_settings();
    
    let cache = cache.lock().await;
    let cache_health = match cache.health().await {
        Ok(_) => "ok",
        Err(_) => "error",
    };
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "version": "0.1.0",
        "message": "These are not the droids you are looking for",
        "environment": settings.environment,
        "network": settings.network.to_string(),
        "services": {
            "cache": cache_health,
        }
    }))
}
