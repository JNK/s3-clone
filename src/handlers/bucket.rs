use actix_web::{web, HttpResponse, HttpRequest};
use log::{info, error, debug};
use quick_xml::se::to_string;
use serde::Serialize;

use crate::auth::{verify_aws_signature, check_permission};
use crate::config::Config;
use crate::error::{access_denied_error, no_such_bucket_error, internal_error};
use crate::storage::Storage;

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

pub async fn list_objects(
    req: HttpRequest,
    path: web::Path<String>,
    config: web::Data<Config>,
    storage: web::Data<Storage>,
) -> HttpResponse {
    let bucket_name = path.into_inner();
    debug!("Listing objects in bucket: {}", bucket_name);

    // Verify AWS signature
    let access_key = match verify_aws_signature(&req, &config).await {
        Ok(key) => key,
        Err(e) => {
            error!("Authentication failed: {}", e.message);
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