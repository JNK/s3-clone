use actix_web::{web, HttpRequest, HttpResponse, Error};
use bytes::Bytes;
use chrono::Utc;
use futures::StreamExt;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use quick_xml::se::to_string;

use crate::auth::{verify_aws_signature, check_permission};
use crate::config::Config;
use crate::storage::{StorageManager, ObjectMetadata};

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "ListBucketResult")]
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

pub async fn list_buckets(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<StorageManager>>,
) -> Result<HttpResponse, Error> {
    let access_key = verify_aws_signature(&req, &config).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.message))?;

    if !check_permission(&config, &access_key, "ListBuckets", "*") {
        return Err(actix_web::error::ErrorForbidden("Permission denied"));
    }

    let buckets = storage.list_buckets()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.message))?;

    let owner = OwnerResponse {
        id: access_key.clone(),
        display_name: access_key,
    };

    let buckets: Vec<BucketResponse> = buckets.into_iter()
        .map(|name| {
            BucketResponse {
                name,
                creation_date: Utc::now().to_rfc3339(),
            }
        })
        .collect();

    let response = ListBucketsResponse {
        owner,
        buckets: BucketsResponse { buckets },
    };

    let xml = to_string(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type("application/xml")
        .body(xml))
}

pub async fn list_objects(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<StorageManager>>,
    path: web::Path<String>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let bucket = path.into_inner();
    let access_key = verify_aws_signature(&req, &config).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.message))?;

    if !check_permission(&config, &access_key, "ListObjects", &bucket) {
        return Err(actix_web::error::ErrorForbidden("Permission denied"));
    }

    let prefix = query.get("prefix").map(String::as_str);
    let objects = storage.list_objects(&bucket, prefix)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.message))?;

    let contents: Vec<ObjectResponse> = objects.into_iter()
        .map(|obj: ObjectMetadata| {
            ObjectResponse {
                key: obj.key,
                size: obj.size as i64,
                last_modified: obj.last_modified.to_rfc3339(),
                e_tag: format!("\"{:x}\"", obj.size),
                storage_class: "STANDARD".to_string(),
                owner: OwnerResponse {
                    id: "".to_string(),
                    display_name: "".to_string(),
                },
            }
        })
        .collect();

    let response = ListObjectsResponse {
        name: bucket.clone(),
        prefix: prefix.unwrap_or("").to_string(),
        delimiter: "/".to_string(),
        max_keys: 1000,
        is_truncated: false,
        contents,
        common_prefixes: Vec::new(),
    };

    let xml = to_string(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type("application/xml")
        .body(xml))
}

pub async fn list_objects_v2(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<StorageManager>>,
    path: web::Path<String>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let bucket = path.into_inner();
    let access_key = verify_aws_signature(&req, &config).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.message))?;

    if !check_permission(&config, &access_key, "ListObjectsV2", &bucket) {
        return Err(actix_web::error::ErrorForbidden("Permission denied"));
    }

    let prefix = query.get("prefix").map(String::as_str);
    let delimiter = query.get("delimiter").map(String::as_str).unwrap_or("/");
    let max_keys = query.get("max-keys")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(1000);

    let objects = storage.list_objects(&bucket, prefix)
        .map_err(|e| {
            log::error!("Failed to list objects: {}", e);
            actix_web::error::ErrorInternalServerError(e.message)
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
            contents.push(ObjectResponse {
                key: obj.key,
                size: obj.size as i64,
                last_modified: obj.last_modified.to_rfc3339(),
                e_tag: format!("\"{:x}\"", obj.size),
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
    storage: web::Data<Arc<StorageManager>>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let (bucket, key) = path.into_inner();
    let access_key = verify_aws_signature(&req, &config).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.message))?;

    if !check_permission(&config, &access_key, "GetObject", &format!("{}/{}", bucket, key)) {
        return Err(actix_web::error::ErrorForbidden("Permission denied"));
    }

    let stream = storage.get_object(bucket, key).await
        .map_err(|e| actix_web::error::ErrorNotFound(e.message))?;

    Ok(HttpResponse::Ok()
        .streaming(stream.map(|chunk| chunk.map(Bytes::from).map_err(|e| actix_web::error::ErrorInternalServerError(e)))))
}

pub async fn put_object(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<StorageManager>>,
    path: web::Path<(String, String)>,
    body: Bytes,
) -> Result<HttpResponse, Error> {
    let (bucket, key) = path.into_inner();
    let access_key = verify_aws_signature(&req, &config).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.message))?;

    if !check_permission(&config, &access_key, "PutObject", &format!("{}/{}", bucket, key)) {
        return Err(actix_web::error::ErrorForbidden("Permission denied"));
    }

    storage.put_object(&bucket, &key, &body)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.message))?;

    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(&body);
    let etag = hex::encode(hasher.finalize());

    Ok(HttpResponse::Ok()
        .append_header(("ETag", format!("\"{}\"", etag)))
        .finish())
}

pub async fn upload_part(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<StorageManager>>,
    path: web::Path<(String, String)>,
    query: web::Query<std::collections::HashMap<String, String>>,
    body: Bytes,
) -> Result<HttpResponse, Error> {
    let (bucket, key) = path.into_inner();
    let access_key = verify_aws_signature(&req, &config).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.message))?;

    if !check_permission(&config, &access_key, "UploadPart", &format!("{}/{}", bucket, key)) {
        return Err(actix_web::error::ErrorForbidden("Permission denied"));
    }

    let part_number = query.get("partNumber")
        .and_then(|s| s.parse::<u32>().ok())
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing or invalid partNumber"))?;

    let etag = storage.upload_part(&bucket, &key, part_number, &body)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.message))?;

    Ok(HttpResponse::Ok()
        .append_header(("ETag", format!("\"{}\"", etag)))
        .finish())
}

pub async fn create_bucket(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<StorageManager>>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let bucket = path.into_inner();
    let access_key = verify_aws_signature(&req, &config).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.message))?;

    if !check_permission(&config, &access_key, "CreateBucket", &bucket) {
        return Err(actix_web::error::ErrorForbidden("Permission denied"));
    }

    storage.create_bucket(&bucket)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.message))?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn head_object(
    req: HttpRequest,
    config: web::Data<Arc<Config>>,
    storage: web::Data<Arc<StorageManager>>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let (bucket, key) = path.into_inner();
    let access_key = verify_aws_signature(&req, &config).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.message))?;

    if !check_permission(&config, &access_key, "HeadObject", &format!("{}/{}", bucket, key)) {
        return Err(actix_web::error::ErrorForbidden("Permission denied"));
    }

    let metadata = storage.head_object(&bucket, &key)
        .map_err(|e| actix_web::error::ErrorNotFound(e.message))?;

    Ok(HttpResponse::Ok()
        .append_header(("Content-Length", metadata.size.to_string()))
        .append_header(("Last-Modified", metadata.last_modified.to_rfc2822()))
        .append_header(("Content-Type", metadata.content_type))
        .append_header(("ETag", format!("\"{:x}\"", metadata.size)))
        .finish())
} 