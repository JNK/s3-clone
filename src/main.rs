mod auth;
mod config;
mod error;
mod handlers;
mod middleware;
mod storage;

use actix_web::{web, App, HttpServer};
use env_logger::Env;
use log::info;
use std::env;

use config::Config;
use storage::Storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load configuration
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());
    let config = Config::load(&config_path).expect("Failed to load configuration");
    let host = config.server.host.clone();
    let port = config.server.port;
    let config_data = web::Data::new(config);

    // Initialize storage
    let storage = Storage::new();
    let storage_data = web::Data::new(storage);

    info!("Starting server at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::request_id::RequestId)
            .app_data(config_data.clone())
            .app_data(storage_data.clone())
            .service(
                web::resource("/")
                    .route(web::get().to(handlers::list_buckets))
            )
            .service(
                web::resource("/{bucket}")
                    .route(web::get().to(handlers::list_objects))
                    .route(web::put().to(handlers::create_bucket))
            )
            .service(
                web::resource("/{bucket}?list-type=2")
                    .route(web::get().to(handlers::list_objects_v2))
            )
            .service(
                web::resource("/{bucket}/{key:.*}")
                    .route(web::get().to(handlers::get_object))
                    .route(web::put().to(handlers::put_object))
                    .route(web::head().to(handlers::head_object))
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
} 