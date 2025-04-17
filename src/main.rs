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
    let config_data = web::Data::new(config);

    // Initialize storage
    let storage = Storage::new();
    let storage_data = web::Data::new(storage);

    info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::request_id::RequestId)
            .app_data(config_data.clone())
            .app_data(storage_data.clone())
            .service(
                web::scope("/{bucket}")
                    .route("", web::get().to(handlers::bucket::list_objects))
                    .service(
                        web::scope("/{key}")
                            .route("", web::get().to(handlers::object::get_object))
                            .route("", web::put().to(handlers::object::put_object))
                            .route("", web::delete().to(handlers::object::delete_object))
                            .route("", web::head().to(handlers::object::head_object)),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 