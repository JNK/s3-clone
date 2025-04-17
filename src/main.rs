mod auth;
mod config;
mod handlers;
mod storage;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Load configuration
    let config = config::Config::load("config.yaml")
        .expect("Failed to load configuration");
    let host = config.server.host.clone();
    let port = config.server.port;
    let config = Arc::new(config);

    // Initialize storage manager
    let storage = storage::StorageManager::new(&config.server.storage_path)
        .expect("Failed to initialize storage manager");
    let storage = Arc::new(storage);

    // Create storage directory if it doesn't exist
    std::fs::create_dir_all(&config.server.storage_path)?;

    println!("Starting S3-compatible server on {}:{}", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&config)))
            .app_data(web::Data::new(Arc::clone(&storage)))
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
            )
            .service(
                web::resource("/{bucket}/{key:.*}?uploadId={upload_id}&partNumber={part_number}")
                    .route(web::put().to(handlers::upload_part))
            )
    })
    .bind((host, port))?
    .run()
    .await
} 