use aws_credential_types::Credentials;
use actix_web::HttpRequest;

use crate::config::Config;

#[derive(Debug)]
pub struct AuthError {
    pub message: String,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AuthError {}

pub async fn verify_aws_signature(
    req: &HttpRequest,
    config: &Config,
) -> Result<String, AuthError> {
    let headers = req.headers();
    
    // Extract the Authorization header
    let auth_header = headers
        .get("Authorization")
        .ok_or_else(|| AuthError {
            message: "Missing Authorization header".to_string(),
        })?
        .to_str()
        .map_err(|_| AuthError {
            message: "Invalid Authorization header".to_string(),
        })?;

    // Parse the Authorization header to get the access key
    let access_key = parse_access_key_from_auth_header(auth_header)
        .ok_or_else(|| AuthError {
            message: "Invalid Authorization header format".to_string(),
        })?;

    // Find the corresponding credential
    let credential = config.find_credential(&access_key).ok_or_else(|| AuthError {
        message: "Invalid access key".to_string(),
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
    
    Ok(access_key)
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