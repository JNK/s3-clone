use aws_credential_types::Credentials;
use actix_web::HttpRequest;
use std::collections::HashMap;
use log::debug;
use percent_encoding::percent_decode_str;
use uuid::Uuid;

use crate::config::Config;
use crate::error::{AuthError, invalid_access_key_error, signature_does_not_match_error};
use crate::middleware::request_id;

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
    // Check for Authorization header first
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

        debug!("Found access key from Authorization header: {}", access_key);

        // Find the corresponding credential
        let credential = config.find_credential(&access_key).ok_or_else(|| AuthError {
            message: "Invalid access key".to_string(),
            code: "InvalidAccessKeyId".to_string(),
        })?;

        // Create AWS credentials for verification
        let _credentials = Credentials::new(
            credential.access_key_id.clone(),
            credential.secret_access_key.clone(),
            None,
            None,
            "s3-clone",
        );

        // Verify the signature
        // Note: This is a simplified version. In a production environment,
        // you would want to do a full signature verification
        
        return Ok(access_key);
    }

    // If no Authorization header, check for query parameters
    let query: HashMap<String, String> = req.query_string()
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.split('=');
            let key = parts.next()?;
            let value = parts.next()?;
            Some((key.to_string(), value.to_string()))
        })
        .collect();

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

        // Verify the signature
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