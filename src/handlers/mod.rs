use actix_web::{web, HttpRequest, HttpResponse, Error};
use bytes::Bytes;
use chrono::Utc;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use quick_xml::se::to_string;
use log::{error, debug};

use crate::auth::{verify_aws_signature, check_permission};
use crate::config::Config;
use crate::error::{access_denied_error, no_such_bucket_error, internal_error};
use crate::storage::Storage;

pub mod bucket;
pub mod object;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "ListAllMyBucketsResult")]
struct ListBucketsResponse {
    #[serde(rename = "Owner")]
    owner: OwnerResponse,
    #[serde(rename = "Buckets")]
    buckets: BucketsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
struct BucketsResponse {
    #[serde(rename = "Bucket")]
    buckets: Vec<BucketResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BucketResponse {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "CreationDate")]
    creation_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OwnerResponse {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "DisplayName")]
    display_name: String,
}

#[derive(Serialize)]
struct ListBucketResult {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Prefix")]
    prefix: Option<String>,
    #[serde(rename = "Marker")]
    marker: Option<String>,
    #[serde(rename = "MaxKeys")]
    max_keys: i32,
    #[serde(rename = "IsTruncated")]
    is_truncated: bool,
    #[serde(rename = "Contents")]
    contents: Vec<Object>,
}

#[derive(Serialize)]
struct Object {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "LastModified")]
    last_modified: String,
    #[serde(rename = "ETag")]
    etag: String,
    #[serde(rename = "Size")]
    size: i64,
    #[serde(rename = "StorageClass")]
    storage_class: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommonPrefix {
    #[serde(rename = "Prefix")]
    prefix: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ObjectResponse {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Size")]
    size: i64,
    #[serde(rename = "LastModified")]
    last_modified: String,
    #[serde(rename = "ETag")]
    e_tag: String,
    #[serde(rename = "StorageClass")]
    storage_class: String,
    #[serde(rename = "Owner")]
    owner: OwnerResponse,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListObjectsResponse {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Prefix")]
    prefix: String,
    #[serde(rename = "Delimiter")]
    delimiter: String,
    #[serde(rename = "MaxKeys")]
    max_keys: i32,
    #[serde(rename = "IsTruncated")]
    is_truncated: bool,
    #[serde(rename = "Contents")]
    contents: Vec<ObjectResponse>,
    #[serde(rename = "CommonPrefixes")]
    common_prefixes: Vec<CommonPrefix>,
}

pub async fn list_buckets(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<Storage>>,
) -> HttpResponse {
    // Verify AWS signature
    let access_key = match verify_aws_signature(&req, &config).await {
        Ok(key) => key,
        Err(e) => {
            error!("Authentication failed: {}", e.to_string());
            return HttpResponse::Forbidden()
                .content_type("application/xml")
                .body(e.to_xml(&req));
        }
    };

    // Check permissions
    if !check_permission(&config, &access_key, "s3:ListAllMyBuckets", "*") {
        error!("Access denied for listing buckets");
        return HttpResponse::Forbidden()
            .content_type("application/xml")
            .body(access_denied_error(&req));
    }

    // List buckets
    match storage.list_buckets() {
        Ok(buckets) => {
            let owner = OwnerResponse {
                id: access_key.clone(),
                display_name: access_key,
            };

            let buckets: Vec<BucketResponse> = buckets.into_iter()
                .map(|name| BucketResponse {
                    name,
                    creation_date: Utc::now().to_rfc3339(),
                })
                .collect();

            let response = ListBucketsResponse {
                owner,
                buckets: BucketsResponse { buckets },
            };

            let xml = match to_string(&response) {
                Ok(xml) => xml,
                Err(e) => {
                    error!("Error serializing response: {}", e);
                    return HttpResponse::InternalServerError()
                        .content_type("application/xml")
                        .body(internal_error(&req, &e.to_string()));
                }
            };

            HttpResponse::Ok()
                .content_type("application/xml")
                .body(xml)
        }
        Err(e) => {
            error!("Error listing buckets: {}", e);
            HttpResponse::InternalServerError()
                .content_type("application/xml")
                .body(internal_error(&req, &e.to_string()))
        }
    }
}

pub async fn list_objects(
    req: HttpRequest,
    path: web::Path<String>,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<Storage>>,
) -> HttpResponse {
    let bucket_name = path.into_inner();
    debug!("Listing objects in bucket: {}", bucket_name);

    // Verify AWS signature
    let access_key = match verify_aws_signature(&req, &config).await {
        Ok(key) => key,
        Err(e) => {
            error!("Authentication failed: {}", e.to_string());
            return HttpResponse::Forbidden()
                .content_type("application/xml")
                .body(e.to_xml(&req));
        }
    };

    // Check permissions
    if !check_permission(&config, &access_key, "s3:ListBucket", &bucket_name) {
        error!("Access denied for bucket: {}", bucket_name);
        return HttpResponse::Forbidden()
            .content_type("application/xml")
            .body(access_denied_error(&req));
    }

    // Check if bucket exists
    if !storage.bucket_exists(&bucket_name) {
        error!("Bucket not found: {}", bucket_name);
        return HttpResponse::NotFound()
            .content_type("application/xml")
            .body(no_such_bucket_error(&req, &bucket_name));
    }

    // Get query parameters
    let query: std::collections::HashMap<String, String> = req.query_string()
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.split('=');
            let key = parts.next()?;
            let value = parts.next()?;
            Some((key.to_string(), value.to_string()))
        })
        .collect();

    let prefix = query.get("prefix").cloned();
    let marker = query.get("marker").cloned();
    let max_keys = query.get("max-keys")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(1000);

    // List objects
    match storage.list_objects(&bucket_name, prefix.as_deref(), marker.as_deref(), max_keys) {
        Ok(objects) => {
            let result = ListBucketResult {
                name: bucket_name,
                prefix,
                marker,
                max_keys,
                is_truncated: false, // TODO: Implement pagination
                contents: objects.into_iter().map(|obj| Object {
                    key: obj.key,
                    last_modified: obj.last_modified,
                    etag: obj.etag,
                    size: obj.size as i64,
                    storage_class: "STANDARD".to_string(),
                }).collect(),
            };

            let mut xml = r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string();
            xml.push_str(&to_string(&result).unwrap_or_else(|_| "".to_string()));

            HttpResponse::Ok()
                .content_type("application/xml")
                .body(xml)
        }
        Err(e) => {
            error!("Error listing objects: {}", e);
            HttpResponse::InternalServerError()
                .content_type("application/xml")
                .body(internal_error(&req, &e.to_string()))
        }
    }
}

pub async fn list_objects_v2(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<Storage>>,
    path: web::Path<String>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let bucket = path.into_inner();
    let access_key = verify_aws_signature(&req, &config).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;

    if !check_permission(&config, &access_key, "ListObjectsV2", &bucket) {
        return Err(actix_web::error::ErrorForbidden("Permission denied"));
    }

    let prefix = query.get("prefix").map(String::as_str);
    let marker = query.get("marker").map(String::as_str);
    let delimiter = query.get("delimiter").map(String::as_str).unwrap_or("/");
    let max_keys = query.get("max-keys")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(1000);

    let objects = storage.list_objects(&bucket, prefix, marker, max_keys)
        .map_err(|e| {
            log::error!("Failed to list objects: {}", e);
            actix_web::error::ErrorInternalServerError(e.to_string())
        })?;

    let mut contents = Vec::new();
    let mut common_prefixes = Vec::new();
    let mut seen_prefixes = std::collections::HashSet::new();

    for obj in objects {
        if let Some(prefix) = prefix {
            if !obj.key.starts_with(prefix) {
                continue;
            }
        }

        if let Some(pos) = obj.key.find(delimiter) {
            let common_prefix = obj.key[..pos + delimiter.len()].to_string();
            if seen_prefixes.insert(common_prefix.clone()) {
                common_prefixes.push(CommonPrefix {
                    prefix: common_prefix,
                });
            }
        } else {
            // Parse last_modified from string (UNIX timestamp) to RFC3339
            let last_modified_rfc3339 = match obj.last_modified.parse::<u64>() {
                Ok(secs) => {
                    use chrono::{TimeZone, Utc};
                    Utc.timestamp_opt(secs as i64, 0).single().map(|dt| dt.to_rfc3339()).unwrap_or(obj.last_modified.clone())
                },
                Err(_) => obj.last_modified.clone(),
            };
            contents.push(ObjectResponse {
                key: obj.key,
                size: obj.size as i64,
                last_modified: last_modified_rfc3339,
                e_tag: obj.etag,
                storage_class: "STANDARD".to_string(),
                owner: OwnerResponse {
                    id: access_key.clone(),
                    display_name: access_key.clone(),
                },
            });
        }
    }

    let response = ListObjectsResponse {
        name: bucket.clone(),
        prefix: prefix.unwrap_or("").to_string(),
        delimiter: delimiter.to_string(),
        max_keys,
        is_truncated: false,
        contents,
        common_prefixes,
    };

    let xml = to_string(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type("application/xml")
        .body(xml))
}

pub async fn get_object(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<Storage>>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let (bucket, key) = path.into_inner();
    let access_key = verify_aws_signature(&req, &config).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;

    if !check_permission(&config, &access_key, "GetObject", &format!("{}/{}", bucket, key)) {
        return Err(actix_web::error::ErrorForbidden("Permission denied"));
    }

    let data = storage.get_object(&bucket, &key)
        .map_err(|e| actix_web::error::ErrorNotFound(e.to_string()))?;

    Ok(HttpResponse::Ok()
        .body(data))
}

pub async fn put_object(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<Storage>>,
    path: web::Path<(String, String)>,
    body: Bytes,
) -> Result<HttpResponse, Error> {
    let (bucket, key) = path.into_inner();
    let access_key = verify_aws_signature(&req, &config).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;

    if !check_permission(&config, &access_key, "PutObject", &format!("{}/{}", bucket, key)) {
        return Err(actix_web::error::ErrorForbidden("Permission denied"));
    }

    storage.put_object(&bucket, &key, body.to_vec())
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(&body);
    let etag = hex::encode(hasher.finalize());

    Ok(HttpResponse::Ok()
        .append_header(("ETag", format!("\"{}\"", etag)))
        .finish())
}

pub async fn create_bucket(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<Storage>>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let bucket = path.into_inner();
    let access_key = verify_aws_signature(&req, &config).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;

    if !check_permission(&config, &access_key, "CreateBucket", &bucket) {
        return Err(actix_web::error::ErrorForbidden("Permission denied"));
    }

    storage.create_bucket(&bucket)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn head_object(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<Storage>>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let (bucket, key) = path.into_inner();
    let access_key = verify_aws_signature(&req, &config).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;

    if !check_permission(&config, &access_key, "HeadObject", &format!("{}/{}", bucket, key)) {
        return Err(actix_web::error::ErrorForbidden("Permission denied"));
    }

    let metadata = storage.head_object(&bucket, &key)
        .map_err(|e| actix_web::error::ErrorNotFound(e.to_string()))?;

    Ok(HttpResponse::Ok()
        .append_header(("Content-Length", metadata.size.to_string()))
        .append_header(("Last-Modified", {
            use chrono::{TimeZone, Utc};
            match metadata.last_modified.parse::<u64>() {
                Ok(secs) => Utc.timestamp_opt(secs as i64, 0).single().map(|dt| dt.to_rfc2822()).unwrap_or(metadata.last_modified.clone()),
                Err(_) => metadata.last_modified.clone(),
            }
        }))
        .append_header(("Content-Type", metadata.content_type.unwrap_or_else(|| "application/octet-stream".to_string())))
        .append_header(("ETag", metadata.etag))
        .finish())
} 