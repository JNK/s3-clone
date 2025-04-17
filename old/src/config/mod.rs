use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use glob::Pattern;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub storage_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub action: String,
    pub resource: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub credentials: Vec<Credential>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn find_credential(&self, access_key: &str) -> Option<&Credential> {
        self.credentials.iter().find(|c| c.access_key_id == access_key)
    }

    pub fn check_permission(&self, access_key: &str, action: &str, resource: &str) -> bool {
        if let Some(credential) = self.find_credential(access_key) {
            credential.permissions.iter().any(|p| {
                let action_matches = p.action == "*" || Pattern::new(&p.action).map(|pat| pat.matches(action)).unwrap_or(false);
                let resource_matches = p.resource == "*" || Pattern::new(&p.resource).map(|pat| pat.matches(resource)).unwrap_or(false);
                action_matches && resource_matches
            })
        } else {
            false
        }
    }
} 