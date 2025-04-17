use std::env;
use std::path::PathBuf;
mod config;
use crate::config::ConfigLoader;

fn main() {
    // Determine config path from first argument or use default
    let args: Vec<String> = env::args().collect();
    let config_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("config.yaml")
    };

    // Initialize ConfigLoader
    let loader = match ConfigLoader::new(&config_path) {
        Ok(loader) => loader,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    println!("Loaded config from {:?}", config_path);

    // Start listening for reload triggers (fsevents, SIGHUP)
    loader.start_listening_for_reloads();

    // Example: print config on reload (in real app, you would update state)
    // This is just to keep the main thread alive
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
