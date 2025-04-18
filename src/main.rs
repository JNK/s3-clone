use std::env;
use std::path::PathBuf;
use tracing_subscriber::{fmt, EnvFilter, reload, layer::SubscriberExt, Registry};
mod config;
use crate::config::{ConfigLoader, LoggingReloadHandle};

fn setup_logging(format: &str, default_level: &str) -> LoggingReloadHandle {
    let env_filter = EnvFilter::try_new(default_level).unwrap();
    let (filter_layer, handle) = reload::Layer::new(env_filter);
    let subscriber: Box<dyn tracing::Subscriber + Send + Sync> = match format {
        "json" => Box::new(Registry::default()
            .with(filter_layer)
            .with(fmt::layer().json())),
        _ => Box::new(Registry::default()
            .with(filter_layer)
            .with(fmt::layer())),
    };
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    LoggingReloadHandle { handle }
}

fn main() {
    // Determine config path from first argument or use default
    let args: Vec<String> = env::args().collect();
    let config_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("config.yaml")
    };

    // Setup logging with initial format and default level (can be hardcoded or from args)
    let initial_format = "text"; // or "json"; could be from args
    let initial_level = "info"; // could be from args
    let reload_handle = setup_logging(initial_format, initial_level);

    // Initialize ConfigLoader
    let loader = match ConfigLoader::new(&config_path) {
        Ok(loader) => loader,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    println!("Loaded config from {:?}", config_path);

    // Update log filter from config
    loader.update_log_filter(&reload_handle);

    // Start listening for reload triggers (fsevents, SIGHUP)
    let loader_clone = loader.clone();
    let reload_handle = std::sync::Arc::new(reload_handle);
    std::thread::spawn(move || {
        loop {
            // In a real app, use a proper reload notification
            std::thread::sleep(std::time::Duration::from_secs(1));
            loader_clone.update_log_filter(&reload_handle);
            println!("Logging configuration re-applied");
        }
    });

    // Keep the main thread alive
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
