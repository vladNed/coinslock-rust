use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/probes/health/")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "version": "0.1.0",
        "message": "These are not the droids you are looking for",
    }))
}