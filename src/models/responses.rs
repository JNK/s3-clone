use hyper::Body;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct S3ErrorResponse {
    pub code: String,
    pub message: String,
    pub request_id: String,
    pub host_id: String,
    pub resource: Option<String>, // e.g., BucketName, Key, etc.
}

#[derive(Debug, Clone)]
pub struct CreateBucketResponse {
    pub location: String,
}

#[derive(Debug, Clone)]
pub struct DeleteBucketResponse;

#[derive(Debug, Clone)]
pub struct ListBucketsResponse {
    pub owner_id: String,
    pub owner_display_name: String,
    pub buckets: Vec<BucketSummary>,
}

#[derive(Debug, Clone)]
pub struct BucketSummary {
    pub name: String,
    pub creation_date: String,
}

#[derive(Debug, Clone)]
pub struct ListObjectsResponse {
    pub name: String,
    pub prefix: Option<String>,
    pub marker: Option<String>,
    pub max_keys: u32,
    pub is_truncated: bool,
    pub contents: Vec<ObjectSummary>,
}

#[derive(Debug, Clone)]
pub struct ListObjectsV2Response {
    pub name: String,
    pub prefix: Option<String>,
    pub key_count: u32,
    pub max_keys: u32,
    pub delimiter: Option<String>,
    pub is_truncated: bool,
    pub contents: Vec<ObjectSummary>,
    pub next_continuation_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ObjectSummary {
    pub key: String,
    pub last_modified: String,
    pub etag: String,
    pub size: u64,
    pub storage_class: String,
}

#[derive(Debug, Clone)]
pub struct GetObjectResponse {
    pub content_type: String,
    pub content_length: u64,
    pub etag: String,
    pub body: Body,
}

#[derive(Debug, Clone)]
pub struct PutObjectResponse {
    pub etag: String,
}

#[derive(Debug, Clone)]
pub struct DeleteObjectResponse;

#[derive(Debug, Clone)]
pub struct InitiateMultipartUploadResponse {
    pub bucket: String,
    pub key: String,
    pub upload_id: String,
}

#[derive(Debug, Clone)]
pub struct UploadPartResponse {
    pub etag: String,
}

#[derive(Debug, Clone)]
pub struct CompleteMultipartUploadResponse {
    pub location: String,
    pub bucket: String,
    pub key: String,
    pub etag: String,
}

#[derive(Debug, Clone)]
pub struct AbortMultipartUploadResponse;

#[derive(Debug, Clone)]
pub enum Response {
    CreateBucket(Result<CreateBucketResponse, S3ErrorResponse>),
    DeleteBucket(Result<DeleteBucketResponse, S3ErrorResponse>),
    ListBuckets(Result<ListBucketsResponse, S3ErrorResponse>),
    ListObjects(Result<ListObjectsResponse, S3ErrorResponse>),
    ListObjectsV2(Result<ListObjectsV2Response, S3ErrorResponse>),
    GetObject(Result<GetObjectResponse, S3ErrorResponse>),
    PutObject(Result<PutObjectResponse, S3ErrorResponse>),
    DeleteObject(Result<DeleteObjectResponse, S3ErrorResponse>),
    InitiateMultipartUpload(Result<InitiateMultipartUploadResponse, S3ErrorResponse>),
    UploadPart(Result<UploadPartResponse, S3ErrorResponse>),
    CompleteMultipartUpload(Result<CompleteMultipartUploadResponse, S3ErrorResponse>),
    AbortMultipartUpload(Result<AbortMultipartUploadResponse, S3ErrorResponse>),
}

// S3 error code constants
pub const ERROR_ACCESS_DENIED: &str = "AccessDenied";
pub const ERROR_NO_SUCH_BUCKET: &str = "NoSuchBucket";
pub const ERROR_NO_SUCH_KEY: &str = "NoSuchKey";
pub const ERROR_NO_SUCH_UPLOAD: &str = "NoSuchUpload";
pub const ERROR_BUCKET_ALREADY_EXISTS: &str = "BucketAlreadyExists";
pub const ERROR_BUCKET_ALREADY_OWNED_BY_YOU: &str = "BucketAlreadyOwnedByYou";
pub const ERROR_BUCKET_NOT_EMPTY: &str = "BucketNotEmpty"; 
pub const ERROR_INVALID_BUCKET_NAME: &str = "InvalidBucketName";
pub const ERROR_INVALID_OBJECT_NAME: &str = "InvalidObjectName";
pub const ERROR_INVALID_PART: &str = "InvalidPart";
pub const ERROR_INVALID_PART_ORDER: &str = "InvalidPartOrder";
pub const ERROR_INVALID_RANGE: &str = "InvalidRange";