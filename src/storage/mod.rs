use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Write};
use walkdir::WalkDir;
use chrono::{DateTime, Utc};
use mime_guess::from_path;
use async_stream::try_stream;
use futures::Stream;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub struct StorageError {
    pub message: String,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StorageError {}

impl From<io::Error> for StorageError {
    fn from(err: io::Error) -> Self {
        StorageError {
            message: err.to_string(),
        }
    }
}

impl From<walkdir::Error> for StorageError {
    fn from(err: walkdir::Error) -> Self {
        StorageError {
            message: err.to_string(),
        }
    }
}

pub struct StorageManager {
    root_path: PathBuf,
}

#[derive(Debug)]
pub struct ObjectMetadata {
    pub key: String,
    pub size: u64,
    pub last_modified: DateTime<Utc>,
    pub content_type: String,
}

impl StorageManager {
    pub fn new<P: AsRef<Path>>(root_path: P) -> io::Result<Self> {
        let root_path = root_path.as_ref().to_path_buf();
        fs::create_dir_all(&root_path)?;
        Ok(StorageManager { root_path })
    }

    pub fn create_bucket(&self, bucket: &str) -> Result<(), StorageError> {
        let bucket_path = self.root_path.join(bucket);
        if bucket_path.exists() {
            return Err(StorageError {
                message: format!("Bucket {} already exists", bucket),
            });
        }
        fs::create_dir_all(&bucket_path)?;
        Ok(())
    }

    pub fn list_buckets(&self) -> Result<Vec<String>, StorageError> {
        let mut buckets = Vec::new();
        for entry in fs::read_dir(&self.root_path)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    buckets.push(name.to_string());
                }
            }
        }
        Ok(buckets)
    }

    pub fn list_objects(&self, bucket: &str, prefix: Option<&str>) -> Result<Vec<ObjectMetadata>, StorageError> {
        let bucket_path = self.root_path.join(bucket);
        if !bucket_path.exists() {
            return Err(StorageError {
                message: format!("Bucket {} does not exist", bucket),
            });
        }

        let mut objects = Vec::new();
        for entry in WalkDir::new(&bucket_path).min_depth(1).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                let relative_path = path.strip_prefix(&bucket_path)
                    .map_err(|_| StorageError {
                        message: "Failed to strip prefix from path".to_string(),
                    })?;
                
                let key = relative_path.to_str()
                    .ok_or_else(|| StorageError {
                        message: "Path contains invalid UTF-8".to_string(),
                    })?
                    .replace("\\", "/");

                if let Some(prefix) = prefix {
                    if !key.starts_with(prefix) {
                        continue;
                    }
                }

                let metadata = entry.metadata()?;
                let last_modified = DateTime::from(metadata.modified()?);
                let content_type = from_path(&path)
                    .first_or_octet_stream()
                    .to_string();

                objects.push(ObjectMetadata {
                    key,
                    size: metadata.len(),
                    last_modified,
                    content_type,
                });
            }
        }
        Ok(objects)
    }

    pub async fn get_object(&self, bucket: String, key: String) -> Result<impl Stream<Item = Result<Vec<u8>, StorageError>> + 'static, StorageError> {
        let path = self.root_path.join(&bucket).join(&key);
        if !path.exists() {
            return Err(StorageError {
                message: format!("Object {}/{} does not exist", bucket, key),
            });
        }

        let file = TokioFile::open(&path).await?;
        let stream = try_stream! {
            let mut file = file;
            let mut buffer = vec![0; 8192];
            loop {
                let n = file.read(&mut buffer).await?;
                if n == 0 {
                    break;
                }
                yield buffer[..n].to_vec();
            }
        };

        Ok(stream)
    }

    pub fn put_object(&self, bucket: &str, key: &str, data: &[u8]) -> Result<(), StorageError> {
        let bucket_path = self.root_path.join(bucket);
        fs::create_dir_all(&bucket_path)?;

        let object_path = bucket_path.join(key);
        if let Some(parent) = object_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(object_path)?;
        file.write_all(data)?;
        Ok(())
    }

    pub fn upload_part(&self, bucket: &str, key: &str, part_number: u32, data: &[u8]) -> Result<String, StorageError> {
        let upload_id = format!("{}-{}", key, part_number);
        let parts_path = self.root_path.join(bucket).join(".parts").join(&upload_id);
        
        if let Some(parent) = parts_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(parts_path)?;
        file.write_all(data)?;

        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        let etag = hex::encode(hasher.finalize());

        Ok(etag)
    }
} 