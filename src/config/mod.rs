// Make sure to add these dependencies in Cargo.toml:
// serde = { version = "1.0.219", features = ["derive"] }
// notify-rs = "4.0.16"
// signal-hook = "0.3.17"

use notify::{Watcher, RecursiveMode, RecommendedWatcher, Config as NotifyConfig, Event, EventKind};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel};
use std::time::Duration;
use sha2::Digest;
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct StorageConfig {
    pub location: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct RegionConfig {
    pub default: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct LoggingConfig {
    pub format: String,
    pub levels: LoggingLevels,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct LoggingLevels {
    pub server: String,
    pub storage: String,
    pub auth: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerConfig {
    pub http: HttpConfig,
    pub https: Option<HttpsConfig>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct HttpConfig {
    pub enabled: bool,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct HttpsConfig {
    pub enabled: bool,
    pub port: u16,
    pub letsencrypt: Option<LetsEncryptConfig>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct LetsEncryptConfig {
    pub enabled: bool,
    pub email: String,
    pub domains: Vec<String>,
    pub do_token: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Credential {
    pub access_key: String,
    pub secret_key: String,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Permission {
    pub action: String,
    pub resource: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DefaultAcls {
    pub public: bool,
    pub allowed_ips: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DefaultCors {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MultipartConfig {
    pub expiry_seconds: u64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ConfigReload {
    pub sighup: bool,
    pub api: bool,
    pub fsevents: bool,
}

pub struct ConfigLoader {
    pub config_path: PathBuf,
    pub config: Arc<Mutex<Config>>,
    reload_active: Arc<AtomicBool>,
}

impl ConfigLoader {
    /// Initialize the loader with a config file path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let config_path = path.as_ref().to_path_buf();
        let config = Config::load_from_file(&config_path)?;
        let config = Arc::new(Mutex::new(config));
        Ok(Self {
            config_path,
            config,
            reload_active: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Reload the config from the file
    pub fn reload(&self) -> Result<bool, String> {
        let new_config = Config::load_from_file(&self.config_path)?;
        let mut cfg = self.config.lock().unwrap();
        if *cfg == new_config {
            // No semantic change
            Ok(false)
        } else {
            *cfg = new_config;
            Ok(true)
        }
    }

    /// Start listening for reload triggers (fsevents and SIGHUP) and call reload() on trigger.
    pub fn start_listening_for_reloads(&self) {
        self.reload_active.store(false, Ordering::SeqCst);
        let (tx, rx) = channel();
        self.reload_active.store(true, Ordering::SeqCst);
        let config_path = self.config_path.clone();
        let reload_active = self.reload_active.clone();
        let config_reload = {
            let cfg = self.config.lock().unwrap();
            cfg.config_reload.clone()
        };
        if config_reload.fsevents {
            let tx_fs = tx.clone();
            let reload_active_fs = reload_active.clone();
            thread::spawn(move || {
                let (notify_tx, notify_rx) = channel();
                let mut watcher = RecommendedWatcher::new(notify_tx, NotifyConfig::default())
                    .expect("Failed to create watcher");
                watcher
                    .watch(config_path.as_ref(), RecursiveMode::NonRecursive)
                    .expect("Failed to start watcher");
                let debounce_window = Duration::from_millis(200);
                let mut pending = false;
                loop {
                    if !reload_active_fs.load(Ordering::SeqCst) {
                        break;
                    }
                    let event = notify_rx.recv();
                    match event {
                        Ok(Ok(Event { kind: EventKind::Modify(_), .. })) => {
                            pending = true;
                        }
                        Ok(_) => {},
                        Err(_) => break,
                    }
                    // Debounce: wait for more events for debounce_window
                    while pending {
                        if notify_rx.recv_timeout(debounce_window).is_ok() {
                            // More events, keep waiting
                        } else {
                            // Debounce window expired, trigger reload
                            let _ = tx_fs.send(());
                            pending = false;
                        }
                    }
                }
            });
        }
        if config_reload.sighup {
            let tx_sighup = tx.clone();
            let reload_active_sighup = reload_active.clone();
            thread::spawn(move || {
                use signal_hook::consts::signal::SIGHUP;
                use signal_hook::iterator::Signals;
                let mut signals = Signals::new(&[SIGHUP]).expect("Failed to register SIGHUP handler");
                for _ in signals.forever() {
                    if !reload_active_sighup.load(Ordering::SeqCst) {
                        break;
                    }
                    let _ = tx_sighup.send(());
                }
            });
        }
        let loader_main = self.clone();
        thread::spawn(move || {
            while loader_main.reload_active.load(Ordering::SeqCst) {
                if rx.recv().is_ok() {
                    match loader_main.reload() {
                        Ok(true) => {
                            println!("Config reloaded");
                            loader_main.start_listening_for_reloads();
                            break;
                        }
                        Ok(false) => {
                            println!("Config unchanged, not reloaded");
                        }
                        Err(e) => {
                            eprintln!("Config reload failed: {}", e);
                        }
                    }
                }
            }
        });
    }
}

impl Clone for ConfigLoader {
    fn clone(&self) -> Self {
        Self {
            config_path: self.config_path.clone(),
            config: Arc::clone(&self.config),
            reload_active: Arc::clone(&self.reload_active),
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

