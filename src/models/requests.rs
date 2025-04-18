use std::collections::HashMap;
use hyper::Body;

#[derive(Debug, Clone)]
pub struct S3CommonHeaders {
    pub date: String,
    pub host: String,
    pub authorization: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PutObjectHeaders {
    pub common: S3CommonHeaders,
    pub content_length: u64,
    pub content_type: Option<String>,
    pub storage_class: Option<String>,
    pub acl: Option<String>,
    pub server_side_encryption: Option<String>,
    pub user_metadata: HashMap<String, String>, // x-amz-meta-*
}

#[derive(Debug, Clone)]
pub struct GetObjectHeaders {
    pub common: S3CommonHeaders,
    pub range: Option<String>,
    pub if_modified_since: Option<String>,
    pub if_unmodified_since: Option<String>,
    pub if_match: Option<String>,
    pub if_none_match: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeleteObjectHeaders {
    pub common: S3CommonHeaders,
}

#[derive(Debug, Clone)]
pub struct CreateBucketHeaders {
    pub common: S3CommonHeaders,
    pub acl: Option<String>,
    pub object_lock_enabled: Option<bool>,
    pub object_ownership: Option<String>,
    pub grant_full_control: Option<String>,
    pub grant_read: Option<String>,
    pub grant_read_acp: Option<String>,
    pub grant_write: Option<String>,
    pub grant_write_acp: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeleteBucketHeaders {
    pub common: S3CommonHeaders,
}

#[derive(Debug, Clone)]
pub struct ListBucketsHeaders {
    pub common: S3CommonHeaders,
}

#[derive(Debug, Clone)]
pub struct ListObjectsHeaders {
    pub common: S3CommonHeaders,
}

#[derive(Debug, Clone)]
pub struct InitiateMultipartUploadHeaders {
    pub common: S3CommonHeaders,
    pub storage_class: Option<String>,
    pub acl: Option<String>,
    pub user_metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct UploadPartHeaders {
    pub common: S3CommonHeaders,
    pub content_length: u64,
    pub content_md5: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CompleteMultipartUploadHeaders {
    pub common: S3CommonHeaders,
}

#[derive(Debug, Clone)]
pub struct AbortMultipartUploadHeaders {
    pub common: S3CommonHeaders,
}

#[derive(Debug, Clone)]
pub struct CreateBucketRequest {
    pub bucket: String,
    pub location_constraint: Option<String>,
    pub headers: CreateBucketHeaders,
}

#[derive(Debug, Clone)]
pub struct DeleteBucketRequest {
    pub bucket: String,
    pub headers: DeleteBucketHeaders,
}

#[derive(Debug, Clone)]
pub struct ListBucketsRequest {
    pub headers: ListBucketsHeaders,
}

#[derive(Debug, Clone)]
pub struct ListObjectsRequest {
    pub bucket: String,
    pub prefix: Option<String>,
    pub delimiter: Option<String>,
    pub marker: Option<String>,
    pub max_keys: Option<u32>,
    pub headers: ListObjectsHeaders,
}

#[derive(Debug, Clone)]
pub struct ListObjectsV2Request {
    pub bucket: String,
    pub prefix: Option<String>,
    pub delimiter: Option<String>,
    pub start_after: Option<String>,
    pub continuation_token: Option<String>,
    pub max_keys: Option<u32>,
    pub headers: ListObjectsHeaders,
}

#[derive(Debug, Clone)]
pub struct PutObjectRequest {
    pub bucket: String,
    pub key: String,
    pub headers: PutObjectHeaders,
    pub body: Body,
}

#[derive(Debug, Clone)]
pub struct GetObjectRequest {
    pub bucket: String,
    pub key: String,
    pub headers: GetObjectHeaders,
}

#[derive(Debug, Clone)]
pub struct DeleteObjectRequest {
    pub bucket: String,
    pub key: String,
    pub headers: DeleteObjectHeaders,
}

#[derive(Debug, Clone)]
pub struct InitiateMultipartUploadRequest {
    pub bucket: String,
    pub key: String,
    pub headers: InitiateMultipartUploadHeaders,
}

#[derive(Debug, Clone)]
pub struct UploadPartRequest {
    pub bucket: String,
    pub key: String,
    pub upload_id: String,
    pub part_number: u32,
    pub headers: UploadPartHeaders,
    pub body: Body,
}

#[derive(Debug, Clone)]
pub struct CompleteMultipartUploadRequest {
    pub bucket: String,
    pub key: String,
    pub upload_id: String,
    pub headers: CompleteMultipartUploadHeaders,
    pub parts: Vec<(u32, String)>, // (part_number, etag)
}

#[derive(Debug, Clone)]
pub struct AbortMultipartUploadRequest {
    pub bucket: String,
    pub key: String,
    pub upload_id: String,
    pub headers: AbortMultipartUploadHeaders,
}

#[derive(Debug, Clone)]
pub enum Request {
    CreateBucket(CreateBucketRequest),
    DeleteBucket(DeleteBucketRequest),
    ListBuckets(ListBucketsRequest),
    ListObjects(ListObjectsRequest),
    ListObjectsV2(ListObjectsV2Request),
    PutObject(PutObjectRequest),
    GetObject(GetObjectRequest),
    DeleteObject(DeleteObjectRequest),
    InitiateMultipartUpload(InitiateMultipartUploadRequest),
    UploadPart(UploadPartRequest),
    CompleteMultipartUpload(CompleteMultipartUploadRequest),
    AbortMultipartUpload(AbortMultipartUploadRequest),
} 