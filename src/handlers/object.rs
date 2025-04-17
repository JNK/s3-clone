use actix_web::{web, HttpResponse, HttpRequest, body::BoxBody};
use actix_web::http::header::{ContentType, ETag, LastModified};
use log::{info, error, debug};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth::{verify_aws_signature, check_permission};
use crate::config::Config;
use crate::error::{AuthError, access_denied_error, no_such_bucket_error, no_such_key_error, method_not_allowed_error, internal_error};
use crate::storage::Storage;

pub async fn get_object(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    config: web::Data<Config>,
    storage: web::Data<Storage>,
) -> HttpResponse {
    let (bucket_name, object_key) = path.into_inner();
    debug!("Getting object: {} from bucket: {}", object_key, bucket_name);

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
    if !check_permission(&config, &access_key, "s3:GetObject", &format!("{}/{}", bucket_name, object_key)) {
        error!("Access denied for object: {} in bucket: {}", object_key, bucket_name);
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
    match storage.get_object(&bucket_name, &object_key) {
        Ok(object) => {
            let mut response = HttpResponse::Ok();
            
            // Set content type if available
            if let Some(content_type) = object.content_type {
                response.content_type(ContentType::parse(&content_type).unwrap_or(ContentType::octet_stream()));
            } else {
                response.content_type(ContentType::octet_stream());
            }

            // Set ETag if available
            if let Some(etag) = object.etag {
                response.insert_header(ETag(etag));
            }

            // Set Last-Modified if available
            if let Ok(last_modified) = object.last_modified.parse::<SystemTime>() {
                response.insert_header(LastModified(last_modified.into()));
            }

            response.body(object.data)
        }
        Err(e) => {
            error!("Error getting object: {}", e);
            if e.to_string().contains("not found") {
                HttpResponse::NotFound()
                    .content_type("application/xml")
                    .body(no_such_key_error(&req, &object_key))
            } else {
                HttpResponse::InternalServerError()
                    .content_type("application/xml")
                    .body(internal_error(&req, &e.to_string()))
            }
        }
    }
}

pub async fn put_object(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    config: web::Data<Config>,
    storage: web::Data<Storage>,
    payload: web::Bytes,
) -> HttpResponse {
    let (bucket_name, object_key) = path.into_inner();
    debug!("Putting object: {} in bucket: {}", object_key, bucket_name);

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
    if !check_permission(&config, &access_key, "s3:PutObject", &format!("{}/{}", bucket_name, object_key)) {
        error!("Access denied for object: {} in bucket: {}", object_key, bucket_name);
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
        .get("Content-Type")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Put object
    match storage.put_object(&bucket_name, &object_key, payload.to_vec(), content_type) {
        Ok(etag) => {
            let mut response = HttpResponse::Ok();
            if let Some(etag) = etag {
                response.insert_header(ETag(etag));
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
    let (bucket_name, object_key) = path.into_inner();
    debug!("Deleting object: {} from bucket: {}", object_key, bucket_name);

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
    if !check_permission(&config, &access_key, "s3:DeleteObject", &format!("{}/{}", bucket_name, object_key)) {
        error!("Access denied for object: {} in bucket: {}", object_key, bucket_name);
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
    match storage.delete_object(&bucket_name, &object_key) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            error!("Error deleting object: {}", e);
            HttpResponse::InternalServerError()
                .content_type("application/xml")
                .body(internal_error(&req, &e.to_string()))
        }
    }
}

pub async fn head_object(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    config: web::Data<Config>,
    storage: web::Data<Storage>,
) -> HttpResponse {
    let (bucket_name, object_key) = path.into_inner();
    debug!("Head object: {} from bucket: {}", object_key, bucket_name);

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
    if !check_permission(&config, &access_key, "s3:GetObject", &format!("{}/{}", bucket_name, object_key)) {
        error!("Access denied for object: {} in bucket: {}", object_key, bucket_name);
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
    match storage.head_object(&bucket_name, &object_key) {
        Ok(object) => {
            let mut response = HttpResponse::Ok();
            
            // Set content type if available
            if let Some(content_type) = object.content_type {
                response.content_type(ContentType::parse(&content_type).unwrap_or(ContentType::octet_stream()));
            } else {
                response.content_type(ContentType::octet_stream());
            }

            // Set ETag if available
            if let Some(etag) = object.etag {
                response.insert_header(ETag(etag));
            }

            // Set Last-Modified if available
            if let Ok(last_modified) = object.last_modified.parse::<SystemTime>() {
                response.insert_header(LastModified(last_modified.into()));
            }

            // Set Content-Length
            response.insert_header(("Content-Length", object.size.to_string()));

            response.finish()
        }
        Err(e) => {
            error!("Error getting object metadata: {}", e);
            if e.to_string().contains("not found") {
                HttpResponse::NotFound()
                    .content_type("application/xml")
                    .body(no_such_key_error(&req, &object_key))
            } else {
                HttpResponse::InternalServerError()
                    .content_type("application/xml")
                    .body(internal_error(&req, &e.to_string()))
            }
        }
    }
} 