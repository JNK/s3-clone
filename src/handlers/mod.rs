use actix_web::{web, HttpRequest, HttpResponse, Error};
use bytes::Bytes;
use chrono::Utc;
use futures::StreamExt;
use std::sync::Arc;
use serde::Serialize;

use crate::auth::{verify_aws_signature, check_permission};
use crate::config::Config;
use crate::storage::{StorageManager, ObjectMetadata};

#[derive(Serialize)]
struct ListBucketsResponse {
    buckets: Vec<BucketResponse>,
    owner: OwnerResponse,
}

#[derive(Serialize)]
struct BucketResponse {
    name: String,
    creation_date: String,
}

#[derive(Serialize)]
struct OwnerResponse {
    id: String,
    display_name: String,
}

#[derive(Serialize)]
struct ListObjectsResponse {
    name: String,
    prefix: String,
    contents: Vec<ObjectResponse>,
}

#[derive(Serialize)]
struct ObjectResponse {
    key: String,
    size: i64,
    last_modified: String,
    e_tag: String,
    storage_class: String,
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
        buckets,
        owner,
    };

    Ok(HttpResponse::Ok().json(response))
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
            }
        })
        .collect();

    let response = ListObjectsResponse {
        name: bucket.clone(),
        prefix: prefix.unwrap_or("").to_string(),
        contents,
    };

    Ok(HttpResponse::Ok().json(response))
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
            }
        })
        .collect();

    let response = ListObjectsResponse {
        name: bucket.clone(),
        prefix: prefix.unwrap_or("").to_string(),
        contents,
    };

    Ok(HttpResponse::Ok().json(response))
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

    let stream = storage.get_object(&bucket, &key).await
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