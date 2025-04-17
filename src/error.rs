use quick_xml::se::to_string;
use serde::Serialize;
use actix_web::HttpRequest;
use uuid::Uuid;

#[derive(Serialize)]
pub struct S3Error {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Resource")]
    resource: Option<String>,
    #[serde(rename = "RequestId")]
    request_id: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    #[serde(rename = "Error")]
    error: S3Error,
}

impl ErrorResponse {
    pub fn new(code: &str, message: &str, resource: Option<&str>, request_id: &str) -> Self {
        ErrorResponse {
            error: S3Error {
                code: code.to_string(),
                message: message.to_string(),
                resource: resource.map(|s| s.to_string()),
                request_id: request_id.to_string(),
            },
        }
    }

    pub fn to_xml(&self) -> String {
        let mut xml = r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string();
        xml.push_str(&to_string(self).unwrap_or_else(|_| "".to_string()));
        xml
    }
}

pub fn access_denied_error(req: &HttpRequest) -> String {
    let request_id = crate::middleware::request_id::get_request_id(req)
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    ErrorResponse::new(
        "AccessDenied",
        "Access Denied",
        None,
        &request_id,
    ).to_xml()
}

pub fn invalid_access_key_error(req: &HttpRequest) -> String {
    let request_id = crate::middleware::request_id::get_request_id(req)
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    ErrorResponse::new(
        "InvalidAccessKeyId",
        "The AWS Access Key Id you provided does not exist in our records.",
        None,
        &request_id,
    ).to_xml()
}

pub fn signature_does_not_match_error(req: &HttpRequest) -> String {
    let request_id = crate::middleware::request_id::get_request_id(req)
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    ErrorResponse::new(
        "SignatureDoesNotMatch",
        "The request signature we calculated does not match the signature you provided. Check your key and signing method.",
        None,
        &request_id,
    ).to_xml()
}

pub fn no_such_bucket_error(req: &HttpRequest, bucket: &str) -> String {
    let request_id = crate::middleware::request_id::get_request_id(req)
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    ErrorResponse::new(
        "NoSuchBucket",
        "The specified bucket does not exist",
        Some(bucket),
        &request_id,
    ).to_xml()
}

pub fn no_such_key_error(req: &HttpRequest, key: &str) -> String {
    let request_id = crate::middleware::request_id::get_request_id(req)
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    ErrorResponse::new(
        "NoSuchKey",
        "The specified key does not exist",
        Some(key),
        &request_id,
    ).to_xml()
}

pub fn method_not_allowed_error(req: &HttpRequest) -> String {
    let request_id = crate::middleware::request_id::get_request_id(req)
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    ErrorResponse::new(
        "MethodNotAllowed",
        "The specified method is not allowed against this resource",
        None,
        &request_id,
    ).to_xml()
}

pub fn internal_error(req: &HttpRequest, message: &str) -> String {
    let request_id = crate::middleware::request_id::get_request_id(req)
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    ErrorResponse::new(
        "InternalError",
        message,
        None,
        &request_id,
    ).to_xml()
} 