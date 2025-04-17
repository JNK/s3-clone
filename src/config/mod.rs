// Make sure to add these dependencies in Cargo.toml:
// serde = { version = "1.0.219", features = ["derive"] }
// notify-rs = "4.0.16"
// signal-hook = "0.3.17"

use serde::{Deserialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use notify::{Watcher, RecursiveMode, RecommendedWatcher, Config as NotifyConfig};
use signal_hook::consts::signal::SIGHUP;
use signal_hook::iterator::Signals;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub storage: StorageConfig,
    pub region: RegionConfig,
    pub logging: LoggingConfig,
    pub server: ServerConfig,
    pub credentials: Vec<Credential>,
    pub default_acls: DefaultAcls,
    pub default_cors: DefaultCors,
    pub multipart: MultipartConfig,
    pub config_reload: ConfigReload,
}


#[derive(Debug, Deserialize, Clone)]
pub struct StorageConfig {
    pub location: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RegionConfig {
    pub default: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub format: String,
    pub levels: LoggingLevels,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingLevels {
    pub server: String,
    pub storage: String,
    pub auth: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub http: HttpConfig,
    pub https: Option<HttpsConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpConfig {
    pub enabled: bool,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpsConfig {
    pub enabled: bool,
    pub port: u16,
    pub letsencrypt: Option<LetsEncryptConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LetsEncryptConfig {
    pub enabled: bool,
    pub email: String,
    pub domains: Vec<String>,
    pub do_token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Credential {
    pub access_key: String,
    pub secret_key: String,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Permission {
    pub action: String,
    pub resource: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DefaultAcls {
    pub public: bool,
    pub allowed_ips: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DefaultCors {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MultipartConfig {
    pub expiry_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigReload {
    pub sighup: bool,
    pub api: bool,
    pub fsevents: bool,
}

pub struct ConfigLoader {
    pub config_path: PathBuf,
    pub config: Arc<Mutex<Config>>,
}

impl ConfigLoader {
    /// Initialize the loader with a config file path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let config_path = path.as_ref().to_path_buf();
        let config = Config::load_from_file(&config_path)?;
        let config = Arc::new(Mutex::new(config));
        Ok(Self { config_path, config })
    }

    /// Reload the config from the file
    pub fn reload(&self) -> Result<(), String> {
        let new_config = Config::load_from_file(&self.config_path)?;
        let mut cfg = self.config.lock().unwrap();
        *cfg = new_config;
        Ok(())
    }

    /// Start listening for reload triggers (fsevents and SIGHUP) and call reload() on trigger.
    pub fn start_listening_for_reloads(&self) {
        let config_reload;
        {
            let cfg = self.config.lock().unwrap();
            config_reload = cfg.config_reload.clone();
        }
        // Filesystem events (notify)
        if config_reload.fsevents {
            let config_path = self.config_path.clone();
            let loader = self.clone();
            thread::spawn(move || {
                let (tx, rx) = std::sync::mpsc::channel();
                let mut watcher = RecommendedWatcher::new(tx, NotifyConfig::default())
                .expect("Failed to create watcher");

                watcher.watch(config_path.as_ref(), RecursiveMode::NonRecursive)
                .expect("Failed to start watcher");

                loop {
                    match rx.recv() {
                        Ok(_) => {
                            if let Err(e) = loader.reload() {
                                eprintln!("Config reload failed: {}", e);
                            } else {
                                println!("Config reloaded from file event");
                            }
                        }
                        Err(e) => {
                            eprintln!("Config watch error: {}", e);
                            break;
                        }
                    }
                }
            });
        }
        // SIGHUP signal
        if config_reload.sighup {
            let loader = self.clone();
            thread::spawn(move || {
                let mut signals = Signals::new(&[SIGHUP]).expect("Failed to register SIGHUP handler");
                for _ in signals.forever() {
                    if let Err(e) = loader.reload() {
                        eprintln!("Config reload failed: {}", e);
                    } else {
                        println!("Config reloaded from SIGHUP");
                    }
                }
            });
        }
    }
}

impl Clone for ConfigLoader {
    fn clone(&self) -> Self {
        Self {
            config_path: self.config_path.clone(),
            config: Arc::clone(&self.config),
        }
    }
}

impl Config {
    /// Load config from file and parse YAML
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        let config: Self = serde_yaml::from_str(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;
        config.validate()?;
        Ok(config)
    }

    /// Validate required fields and value ranges
    pub fn validate(&self) -> Result<(), String> {
        if self.storage.location.is_empty() {
            return Err("storage.location must not be empty".to_string());
        }
        if self.region.default.is_empty() {
            return Err("region.default must not be empty".to_string());
        }
        if self.server.http.port == 0 {
            return Err("server.http.port must be > 0".to_string());
        }
        if let Some(https) = &self.server.https {
            if https.port == 0 {
                return Err("server.https.port must be > 0".to_string());
            }
            if let Some(le) = &https.letsencrypt {
                if le.email.is_empty() || le.domains.is_empty() || le.do_token.is_empty() {
                    return Err("letsencrypt config fields must not be empty".to_string());
                }
            }
        }
        if self.credentials.is_empty() {
            return Err("at least one credential must be defined".to_string());
        }
        for cred in &self.credentials {
            if cred.access_key.is_empty() || cred.secret_key.is_empty() {
                return Err("credential access_key and secret_key must not be empty".to_string());
            }
        }
        if self.multipart.expiry_seconds == 0 {
            return Err("multipart.expiry_seconds must be > 0".to_string());
        }
        Ok(())
    }
} 