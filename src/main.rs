use crate::config::Config;
use log::info;
use std::env;
use std::path::PathBuf;

pub mod config;

fn main() {
    env_logger::init();

    // Example log usage
    info!("Logger initialized");

    // Determine config path from first argument or use default
    let args: Vec<String> = env::args().collect();
    let config_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("config.yaml")
    };

    // Load config (replace with your config loading logic)
    let cfg = Config::load_from_file(&config_path).unwrap();
    info!("Loaded config from {:?}", config_path);

    // ...rest of your app...
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
