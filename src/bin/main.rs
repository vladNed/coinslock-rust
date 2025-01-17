use actix_web::{middleware, web, App, HttpServer};
use coinslock_rust::api::{lockers, probes};
use log::info;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server at localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1")
                    .service(probes::health)
                    .service(lockers::new_locker)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}