use actix_web::{web, HttpResponse, HttpRequest};
use actix_web::http::header::{HeaderValue, EntityTag};
use bytes::Bytes;
use log::{error, debug};
use std::str::FromStr;
use chrono::{DateTime, Utc};

use crate::auth::{verify_aws_signature, check_permission};
use crate::config::Config;
use crate::error::{access_denied_error, no_such_bucket_error, no_such_key_error, method_not_allowed_error, internal_error};
use crate::storage::Storage;

pub async fn get_object(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    config: web::Data<Config>,
    storage: web::Data<Storage>,
) -> HttpResponse {
    let (bucket_name, key) = path.into_inner();
    debug!("Getting object: {}/{}", bucket_name, key);

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
    if !check_permission(&config, &access_key, "s3:GetObject", &bucket_name) {
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

    // Get object
    match storage.get_object(&bucket_name, &key) {
        Ok(data) => {
            let mut response = HttpResponse::Ok();
            
            // Get metadata separately
            if let Ok(metadata) = storage.head_object(&bucket_name, &key) {
                // Set Content-Type
                if let Some(content_type) = metadata.content_type {
                    response.content_type(content_type);
                }

                // Set ETag
                response.insert_header(("ETag", EntityTag::new_strong(metadata.etag)));

                // Set Last-Modified
                if let Ok(dt) = DateTime::<Utc>::from_str(&metadata.last_modified) {
                    response.insert_header(("Last-Modified", dt.to_rfc2822()));
                }

                // Set Content-Length
                response.insert_header(("Content-Length", metadata.size.to_string()));
            }

            response.body(data)
        }
        Err(e) => {
            error!("Error getting object: {}", e);
            HttpResponse::NotFound()
                .content_type("application/xml")
                .body(no_such_key_error(&req, &key))
        }
    }
}

pub async fn head_object(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    config: web::Data<Config>,
    storage: web::Data<Storage>,
) -> HttpResponse {
    let (bucket_name, key) = path.into_inner();
    debug!("HEAD object: {}/{}", bucket_name, key);

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
    if !check_permission(&config, &access_key, "s3:GetObject", &bucket_name) {
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

    // Get object metadata
    match storage.head_object(&bucket_name, &key) {
        Ok(metadata) => {
            let mut response = HttpResponse::Ok();
            
            // Set Content-Type
            if let Some(content_type) = metadata.content_type {
                response.content_type(content_type);
            }

            // Set ETag
            response.insert_header(("ETag", EntityTag::new_strong(metadata.etag)));

            // Set Last-Modified
            if let Ok(dt) = DateTime::<Utc>::from_str(&metadata.last_modified) {
                response.insert_header(("Last-Modified", dt.to_rfc2822()));
            }

            // Set Content-Length
            response.insert_header(("Content-Length", metadata.size.to_string()));

            response.finish()
        }
        Err(e) => {
            error!("Error getting object metadata: {}", e);
            HttpResponse::NotFound()
                .content_type("application/xml")
                .body(no_such_key_error(&req, &key))
        }
    }
}

pub async fn put_object(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    body: Bytes,
    config: web::Data<Config>,
    storage: web::Data<Storage>,
) -> HttpResponse {
    let (bucket_name, key) = path.into_inner();
    debug!("Putting object: {}/{}", bucket_name, key);

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
    if !check_permission(&config, &access_key, "s3:PutObject", &bucket_name) {
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

    // Get content type from headers
    let content_type = req.headers()
        .get("content-type")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    // Put object
    match storage.put_object(&bucket_name, &key, body.to_vec()) {
        Ok(etag) => {
            let mut response = HttpResponse::Ok();
            
            // Set ETag
            if let Some(etag) = etag {
                response.insert_header(("ETag", EntityTag::new_strong(etag)));
            }

            response.finish()
        }
        Err(e) => {
            error!("Error putting object: {}", e);
            HttpResponse::InternalServerError()
                .content_type("application/xml")
                .body(internal_error(&req, &e.to_string()))
        }
    }
}

pub async fn delete_object(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    config: web::Data<Config>,
    storage: web::Data<Storage>,
) -> HttpResponse {
    let (bucket_name, key) = path.into_inner();
    debug!("Deleting object: {}/{}", bucket_name, key);

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
    if !check_permission(&config, &access_key, "s3:DeleteObject", &bucket_name) {
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

    // Delete object
    match storage.delete_object(&bucket_name, &key) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            error!("Error deleting object: {}", e);
            HttpResponse::InternalServerError()
                .content_type("application/xml")
                .body(internal_error(&req, &e.to_string()))
        }
    }
} 