use crate::config::Config;
use axum::{Router, routing::get};
use log::info;

async fn healthz() -> &'static str {
    "OK"
}

pub async fn run(cfg: Config) {
    let app = Router::new()
    .route("/healthz", get(healthz));
    let addr = format!("{}:{}", cfg.server.http.host, cfg.server.http.port);

    info!("Starting HTTP server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}