#[derive(Debug, Clone)]
pub struct Bucket {
    pub name: String,
    // Add more fields as needed
}

#[derive(Debug, Clone)]
pub struct Object {
    pub bucket: String,
    pub key: String,
    // Add more fields as needed
}

#[derive(Debug, Clone)]
pub struct Part {
    pub part_number: u32,
    pub etag: String,
    pub size: u64,
    // Add more fields as needed
}

#[derive(Debug, Clone)]
pub struct BucketMetadata {
    pub name: String,
    pub region: String,
    pub created: String,
    pub created_by: String,
    // ACLs, CORS, etc.
}

#[derive(Debug, Clone)]
pub struct ObjectMetadata {
    pub key: String,
    pub size: u64,
    pub etag: String,
    pub last_modified: String,
    // Add more fields as needed
}

#[derive(Debug, Clone)]
pub struct Permission {
    pub action: String,
    pub resource: String,
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub access_key: String,
    pub secret_key: String,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone)]
pub enum AuthContext {
    Anonymous,
    AwsAccount(Credentials),
} 