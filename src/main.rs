use crate::config::Config;
use log::info;

mod config;
mod server;

#[tokio::main]
async fn main() {
    env_logger::init();
    let cfg = Config::load_from_file("config.yaml").unwrap();
    info!("Loaded config from config.yaml");
    server::run(cfg).await;
}
