mod auth;
mod config;
mod error;
mod handlers;
mod middleware;
mod storage;

use actix_web::{web, App, HttpServer};
use log::info;
use std::env;
use std::sync::Arc;

use config::Config;
use storage::Storage;
use middleware::request_id::RequestId;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());
    let config = Config::from_file(&config_path).expect("Failed to load configuration");
    let config = Arc::new(config);

    // Initialize storage
    let storage = Storage::new(&config.storage_path).expect("Failed to initialize storage");
    let storage = Arc::new(storage);

    info!("Starting S3-compatible server on {}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(RequestId)
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(storage.clone()))
            .service(
                web::resource("/{bucket}")
                    .route(web::get().to(handlers::bucket::list_objects))
            )
            .service(
                web::resource("/{bucket}/{object:.*}")
                    .route(web::get().to(handlers::object::get_object))
                    .route(web::put().to(handlers::object::put_object))
                    .route(web::delete().to(handlers::object::delete_object))
                    .route(web::head().to(handlers::object::head_object))
            )
    })
    .bind((config.host.clone(), config.port))?
    .run()
    .await
} 