use aws_credential_types::Credentials;
use actix_web::HttpRequest;
use std::collections::HashMap;
use log::debug;
use percent_encoding::percent_decode_str;
use chrono::{DateTime, Utc, Duration, NaiveDateTime};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex::encode as hex_encode;

use crate::config::Config;
use crate::error::{invalid_access_key_error, signature_does_not_match_error};

#[derive(Debug)]
pub struct AuthError {
    pub message: String,
    pub code: String,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AuthError {}

impl AuthError {
    pub fn to_xml(&self, req: &HttpRequest) -> String {
        match self.code.as_str() {
            "InvalidAccessKeyId" => invalid_access_key_error(req),
            "SignatureDoesNotMatch" => signature_does_not_match_error(req),
            _ => invalid_access_key_error(req), // Default to invalid access key for other auth errors
        }
    }
}

pub async fn verify_aws_signature(
    req: &HttpRequest,
    config: &Config,
) -> Result<String, AuthError> {
    // Presigned URL expiry enforcement (must be checked for all requests with these params)
    let query: HashMap<String, String> = req.query_string()
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.split('=');
            let key = parts.next()?;
            let value = parts.next()?;
            Some((key.to_string(), value.to_string()))
        })
        .collect();

    if let (Some(expires), Some(amz_date)) = (query.get("X-Amz-Expires"), query.get("X-Amz-Date")) {
        let expires = expires.parse::<i64>();

        // Try RFC3339, then AWS format, always convert to Utc
        let amz_date_parsed = NaiveDateTime::parse_from_str(amz_date, "%Y%m%dT%H%M%SZ")
            .map(|dt| dt.and_utc());
        match (expires, amz_date_parsed) {
            (Ok(expires), Ok(amz_date_utc)) => {
                let expiry_time = amz_date_utc + Duration::seconds(expires);
                let now = Utc::now();
                log::debug!("amz_date_utc: {:?}, expiry_time: {:?}, now: {:?}", amz_date_utc, expiry_time, now);
                if now > expiry_time {
                    log::info!("Presigned URL expired: now = {:?}, expiry_time = {:?}", now, expiry_time);
                    return Err(AuthError {
                        message: "Request has expired".to_string(),
                        code: "AccessDenied".to_string(),
                    });
                }
            }
            (e, d) => {
                log::debug!("Failed to parse expiry or amz_date: expires={:?}, amz_date={:?}, error={:?} date_error={:?}", query.get("X-Amz-Expires"), amz_date, e, d);
            }
        }
    }

    // Presigned URL signature verification (query-based)
    if let (Some(signature), Some(credential), Some(amz_date)) = (
        query.get("X-Amz-Signature"),
        query.get("X-Amz-Credential"),
        query.get("X-Amz-Date")
    ) {
        let decoded_credential = percent_decode_str(credential)
            .decode_utf8()
            .map_err(|_| AuthError {
                message: "Invalid credential encoding".to_string(),
                code: "InvalidAccessKeyId".to_string(),
            })?;
        let parts: Vec<&str> = decoded_credential.split('/').collect();
        if parts.is_empty() {
            return Err(AuthError {
                message: "Invalid credential format".to_string(),
                code: "InvalidAccessKeyId".to_string(),
            });
        }
        let access_key = parts[0].to_string();
        let credential = config.find_credential(&access_key).ok_or_else(|| {
            log::debug!("No credential found for access key: {}", access_key);
            AuthError {
                message: "Invalid access key".to_string(),
                code: "InvalidAccessKeyId".to_string(),
            }
        })?;
        // Build the string to sign (simplified: just canonical query string for demo)
        // In real S3, this is much more complex!
        let mut canonical_query: Vec<(&String, &String)> = query.iter().collect();
        canonical_query.sort_by(|a, b| a.0.cmp(&b.0));
        let canonical_query_str = canonical_query.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>().join("&");
        let string_to_sign = canonical_query_str;
        log::debug!("String to sign: {}", string_to_sign);
        // Derive signing key (simplified: just use secret key)
        let mut mac = Hmac::<Sha256>::new_from_slice(credential.secret_access_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(string_to_sign.as_bytes());
        let computed_signature = hex_encode(mac.finalize().into_bytes());
        log::debug!("Provided signature: {}", signature);
        log::debug!("Computed signature: {}", computed_signature);
        if &computed_signature != signature {
            log::debug!("Signature mismatch: denying access");
            return Err(AuthError {
                message: "Signature does not match".to_string(),
                code: "SignatureDoesNotMatch".to_string(),
            });
        }
        log::debug!("Signature valid for access key: {}", access_key);
        return Ok(access_key);
    }

    // Check for Authorization header first (TODO: implement header-based signature verification)
    if let Some(auth_header) = req.headers().get("Authorization") {
        let auth_header = auth_header
            .to_str()
            .map_err(|_| AuthError {
                message: "Invalid Authorization header".to_string(),
                code: "InvalidAccessKeyId".to_string(),
            })?;
        let access_key = parse_access_key_from_auth_header(auth_header)
            .ok_or_else(|| AuthError {
                message: "Invalid Authorization header format".to_string(),
                code: "InvalidAccessKeyId".to_string(),
            })?;
        log::debug!("Found access key from Authorization header: {}", access_key);
        let credential = config.find_credential(&access_key).ok_or_else(|| AuthError {
            message: "Invalid access key".to_string(),
            code: "InvalidAccessKeyId".to_string(),
        })?;
        // TODO: Implement AWS SigV4 header-based signature verification
        log::debug!("TODO: Signature verification for Authorization header not implemented");
        return Ok(access_key);
    }

    debug!("Query parameters: {:?}", query);
    if let Some(credential) = query.get("X-Amz-Credential") {
        let decoded_credential = percent_decode_str(credential)
            .decode_utf8()
            .map_err(|_| AuthError {
                message: "Invalid credential encoding".to_string(),
                code: "InvalidAccessKeyId".to_string(),
            })?;
        let parts: Vec<&str> = decoded_credential.split('/').collect();
        debug!("Credential parts: {:?}", parts);
        if parts.is_empty() {
            return Err(AuthError {
                message: "Invalid credential format".to_string(),
                code: "InvalidAccessKeyId".to_string(),
            });
        }
        let access_key = parts[0].to_string();
        debug!("Extracted access key from query: {}", access_key);
        let credential = config.find_credential(&access_key).ok_or_else(|| {
            debug!("No credential found for access key: {}", access_key);
            AuthError {
                message: "Invalid access key".to_string(),
                code: "InvalidAccessKeyId".to_string(),
            }
        })?;
        // Create AWS credentials for verification
        let _credentials = Credentials::new(
            credential.access_key_id.clone(),
            credential.secret_access_key.clone(),
            None,
            None,
            "s3-clone",
        );
        // Note: This is a simplified version. In a production environment,
        // you would want to do a full signature verification
        return Ok(access_key);
    }
    Err(AuthError {
        message: "Missing authorization".to_string(),
        code: "InvalidAccessKeyId".to_string(),
    })
}

fn parse_access_key_from_auth_header(auth_header: &str) -> Option<String> {
    // Example header: AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request, ...
    let parts: Vec<&str> = auth_header.split("Credential=").collect();
    if parts.len() != 2 {
        return None;
    }

    let credential_parts: Vec<&str> = parts[1].split('/').collect();
    if credential_parts.is_empty() {
        return None;
    }

    Some(credential_parts[0].to_string())
}

pub fn check_permission(config: &Config, access_key: &str, action: &str, resource: &str) -> bool {
    config.check_permission(access_key, action, resource)
} 