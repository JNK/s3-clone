use std::path::PathBuf;
use std::fs;
use std::io;
use std::time::UNIX_EPOCH;
use log::error;
use mime_guess::from_path;
use sha2::{Sha256, Digest};
use hex;
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Object not found: {0}")]
    NotFound(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectMetadata {
    pub key: String,
    pub size: u64,
    pub last_modified: String,
    pub etag: String,
    pub content_type: Option<String>,
}

pub struct Storage {
    base_path: PathBuf,
}

impl Storage {
    pub fn new() -> Self {
        let base_path = PathBuf::from("storage");
        if !base_path.exists() {
            fs::create_dir_all(&base_path).expect("Failed to create storage directory");
        }
        Storage { base_path }
    }

    pub fn bucket_exists(&self, bucket_name: &str) -> bool {
        let bucket_path = self.base_path.join(bucket_name);
        bucket_path.exists() && bucket_path.is_dir()
    }

    pub fn create_bucket(&self, bucket_name: &str) -> Result<(), StorageError> {
        let bucket_path = self.base_path.join(bucket_name);
        fs::create_dir_all(bucket_path)?;
        Ok(())
    }

    pub fn delete_bucket(&self, bucket_name: &str) -> Result<(), StorageError> {
        let bucket_path = self.base_path.join(bucket_name);
        if !bucket_path.exists() {
            return Err(StorageError::NotFound(format!("Bucket {} not found", bucket_name)));
        }
        fs::remove_dir_all(bucket_path)?;
        Ok(())
    }

    pub fn list_objects(
        &self,
        bucket_name: &str,
        prefix: Option<&str>,
        marker: Option<&str>,
        max_keys: i32,
    ) -> Result<Vec<ObjectMetadata>, StorageError> {
        let bucket_path = self.base_path.join(bucket_name);
        if !bucket_path.exists() {
            return Err(StorageError::NotFound(format!("Bucket {} not found", bucket_name)));
        }

        let mut objects = Vec::new();
        let prefix = prefix.unwrap_or("");
        let marker = marker.unwrap_or("");

        for entry in fs::read_dir(bucket_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let key = path.file_name().unwrap().to_string_lossy().to_string();
                if key.starts_with(prefix) && key.as_str() > marker {
                    if let Ok(metadata) = self.get_object_metadata(bucket_name, &key) {
                        objects.push(metadata);
                        if objects.len() >= max_keys as usize {
                            break;
                        }
                    }
                }
            }
        }

        Ok(objects)
    }

    pub fn get_object(&self, bucket_name: &str, key: &str) -> Result<Vec<u8>, StorageError> {
        let object_path = self.base_path.join(bucket_name).join(key);
        if !object_path.exists() {
            return Err(StorageError::NotFound(format!("Object {} not found", key)));
        }
        Ok(fs::read(object_path)?)
    }

    pub fn put_object(
        &self,
        bucket_name: &str,
        key: &str,
        data: Vec<u8>,
        // content_type: Option<String>,
    ) -> Result<Option<String>, StorageError> {
        let bucket_path = self.base_path.join(bucket_name);
        if !bucket_path.exists() {
            return Err(StorageError::NotFound(format!("Bucket {} not found", bucket_name)));
        }

        let object_path = bucket_path.join(key);
        fs::write(&object_path, &data)?;

        // Calculate ETag (simplified version using SHA-256)
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let etag = format!("\"{}\"", hex::encode(hasher.finalize()));

        Ok(Some(etag))
    }

    pub fn delete_object(&self, bucket_name: &str, key: &str) -> Result<(), StorageError> {
        let object_path = self.base_path.join(bucket_name).join(key);
        if !object_path.exists() {
            return Err(StorageError::NotFound(format!("Object {} not found", key)));
        }
        fs::remove_file(object_path)?;
        Ok(())
    }

    pub fn head_object(&self, bucket_name: &str, key: &str) -> Result<ObjectMetadata, StorageError> {
        let object_path = self.base_path.join(bucket_name).join(key);
        if !object_path.exists() {
            return Err(StorageError::NotFound(format!("Object {} not found", key)));
        }
        self.get_object_metadata(bucket_name, key)
    }

    fn get_object_metadata(&self, bucket_name: &str, key: &str) -> Result<ObjectMetadata, StorageError> {
        let object_path = self.base_path.join(bucket_name).join(key);
        let metadata = fs::metadata(&object_path)?;
        
        let last_modified = metadata.modified()?
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let data = fs::read(&object_path)?;
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let etag = format!("\"{}\"", hex::encode(hasher.finalize()));

        let content_type = from_path(&object_path)
            .first()
            .map(|mime| mime.to_string());

        Ok(ObjectMetadata {
            key: key.to_string(),
            size: metadata.len(),
            last_modified: last_modified.to_string(),
            etag,
            content_type,
        })
    }

    pub fn list_buckets(&self) -> Result<Vec<String>, StorageError> {
        let mut buckets = Vec::new();
        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    buckets.push(name.to_string_lossy().to_string());
                }
            }
        }
        Ok(buckets)
    }
} 