use std::sync::Arc;

use actix_web::{middleware, web, App, HttpServer};
use coinslock_rust::api::{lockers, probes};
use coinslock_rust::storage::cache;
use dotenv::dotenv;
use log::info;
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting server at localhost:8080");

    let cache = Arc::new(Mutex::new(cache::CacheClient::new().unwrap()));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(cache.clone()))
            .service(
                web::scope("/api/v1")
                    .service(probes::health)
                    .service(lockers::new_locker)
                    .service(lockers::save_locker),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
