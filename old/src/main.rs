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
use std::sync::{Arc, RwLock};
use notify::{RecommendedWatcher, RecursiveMode, Watcher, EventKind};
use std::sync::mpsc::channel;
use std::thread;
use std::path::Path;

use config::Config;
use storage::Storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load configuration
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());
    let config = Arc::new(RwLock::new(Config::load(&config_path).expect("Failed to load configuration")));

    // Spawn a thread to watch the config file and reload on change
    {
        let config_path = config_path.clone();
        let config = Arc::clone(&config);
        thread::spawn(move || {
            let (tx, rx) = channel();
            let mut watcher = notify::recommended_watcher(tx).expect("Failed to create watcher");
            watcher.watch(Path::new(&config_path), RecursiveMode::NonRecursive).expect("Failed to watch config file");
            let mut last_reload = std::time::Instant::now() - std::time::Duration::from_secs(1);
            loop {
                match rx.recv() {
                    Ok(event) => {
                        if let notify::EventKind::Modify(_) = event.unwrap().kind {
                            let now = std::time::Instant::now();
                            if now.duration_since(last_reload) > std::time::Duration::from_millis(500) {
                                last_reload = now;
                                match Config::load(&config_path) {
                                    Ok(new_config) => {
                                        let mut cfg = config.write().unwrap();
                                        *cfg = new_config;
                                        log::info!("Reloaded config from {}", config_path);
                                    }
                                    Err(e) => {
                                        log::error!("Failed to reload config: {}", e);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Watch error: {}", e);
                        break;
                    }
                }
            }
        });
    }

    let host = config.read().unwrap().server.host.clone();
    let port = config.read().unwrap().server.port;
    let config_data = web::Data::new(config);

    // Initialize storage
    let storage = Arc::new(Storage::new());
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