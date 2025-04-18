use log::debug;
use serde::Deserialize;
use std::cmp::PartialEq;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub storage: StorageConfig,
    pub region: RegionConfig,
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
pub struct ServerConfig {
    pub http: HttpConfig,
    pub https: Option<HttpsConfig>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct HttpConfig {
    pub enabled: bool,
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct HttpsConfig {
    pub enabled: bool,
    pub port: u16,
    pub letsencrypt: Option<LetsEncryptConfig>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct LetsEncryptConfig {
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

impl Config {
    /// Load config from file and parse YAML
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        debug!("Loading config from {:?}", path.as_ref());
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        let config: Self = serde_yaml::from_str(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;
        config.validate()?;
        Ok(config)
    }

    /// Validate required fields and value ranges
    pub fn validate(&self) -> Result<(), String> {
        debug!("validating config");
        if self.storage.location.is_empty() {
            debug!("storage.location is empty");
            return Err("storage.location must not be empty".to_string());
        }

        if self.region.default.is_empty() {
            debug!("region.default is empty");
            return Err("region.default must not be empty".to_string());
        }
        if self.server.http.port == 0 {
            debug!("server.http.port is 0");
            return Err("server.http.port must be > 0".to_string());
        }
        if let Some(https) = &self.server.https {
            if https.port == 0 {
                debug!("server.https.port is 0");
                return Err("server.https.port must be > 0".to_string());
            }
            if let Some(le) = &https.letsencrypt {
                if le.email.is_empty() || le.domains.is_empty() || le.do_token.is_empty() {
                    debug!("letsencrypt config fields must not be empty");
                    return Err("letsencrypt config fields must not be empty".to_string());
                }
            }
        }
        if self.credentials.is_empty() {
            debug!("credentials must not be empty");
            return Err("at least one credential must be defined".to_string());
        }
        for cred in &self.credentials {
            if cred.access_key.is_empty() || cred.secret_key.is_empty() {
                debug!("credential access_key and secret_key must not be empty");
                return Err("credential access_key and secret_key must not be empty".to_string());
            }
        }
        if self.multipart.expiry_seconds == 0 {
            debug!("multipart.expiry_seconds must be > 0");
            return Err("multipart.expiry_seconds must be > 0".to_string());
        }

        debug!("config is valid");

        Ok(())
    }
}
