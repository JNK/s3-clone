# s3-clone: S3-Compatible Local Storage Server in Rust

## Overview

`s3-clone` is a local S3-compatible object storage server, using the filesystem for storage and supporting the S3 REST API, AWSv4 signatures, and presigned URLs.  
**API and models are self-documenting** using Rust doc comments and OpenAPI annotations.

---

## Features

- S3 REST API compatibility (buckets, objects, multipart, byte-range, presigned URLs)
- Local directory storage (configurable location)
- Multiple credentials with IAM-like permissions (YAML config, hot-reload)
- Bucket ACLs (public, IP, CORS)
- AWSv4 signature support
- Correct XML responses
- Structured, per-component logging (text/JSON, request IDs, context)
- Healthcheck endpoint
- Optional TLS (Let's Encrypt via DigitalOcean DNS)
- Basic integration tests
- OpenAPI/Swagger docs for all endpoints at `/_docs`

---

## Implementation Notes

**Host and Path Style:**
- All API requests are served from `localhost` (or the configured bind address).
- Path-style URLs are used for all operations, e.g.:
  - `PUT /bucket-name`
  - `GET /bucket-name/object-key`
- No virtual-hosted style (e.g., `bucket.localhost`) is used.

**Default Region:**
- The default region is `de-muc-01` (configurable via the config file).
- If a region is not specified in a request, this default is used.

**Optional Headers:**
- Optional headers such as ACLs, object lock, and ownership are **deprioritized** and will be ignored unless they relate to a feature being actively built.

---

## Implementation Steps (Detailed)

### 1. Project Initialization
- [x] Set up Rust project and dependencies.
- [x] Create initial directory structure.

---

### 2. API Research & Self-Documenting Code

#### 2.1. General
- [x] Research AWS S3 API documentation for all relevant endpoints and XML schemas (in progress, see below for completed endpoints).
- [x] For each method, document request/response structure, edge cases, and validation requirements (in progress).

#### 2.2. Bucket Operations
- [x] Research and document:
    - [x] Create Bucket (`PUT /{bucket}`)
    - [x] List Buckets (`GET /`)
    - [x] Delete Bucket (`DELETE /{bucket}`)
    - [x] List Objects in Bucket (`GET /{bucket}`)
    - [x] List Objects V2 (`GET /{bucket}?list-type=2`)

#### 2.3. Object Operations
- [x] Research and document:
    - [x] Put Object (`PUT /{bucket}/{object}`)
    - [x] Get Object (`GET /{bucket}/{object}`)
    - [x] Get Object (Byte Range) (`GET /{bucket}/{object}` with `Range` header)
    - [x] Delete Object (`DELETE /{bucket}/{object}`)

#### 2.4. Multipart Uploads
- [x] Research and document:
    - [x] Initiate Multipart Upload (`POST /{bucket}/{object}?uploads`)
    - [x] Upload Part (`PUT /{bucket}/{object}?partNumber={PartNumber}&uploadId={UploadId}`)
    - [x] Complete Multipart Upload (`POST /{bucket}/{object}?uploadId={UploadId}`)
    - [x] Abort Multipart Upload (`DELETE /{bucket}/{object}?uploadId={UploadId}`)
    - [x] List Multipart Uploads (`GET /{bucket}?uploads`)
    - [x] List Parts (`GET /{bucket}/{object}?uploadId={UploadId}`)

#### 2.5. Presigned URLs
- [x] Research and document:
    - [x] Presigned GET Object
    - [x] Presigned PUT Object

#### 2.6. ACLs & CORS
- [x] Research and document:
    - [x] Bucket ACLs (public, IP-based)
    - [x] CORS preflight (`OPTIONS /{bucket}/{object}`)

#### 2.7. Healthcheck & Misc
- [x] Research and document:
    - [x] Healthcheck (`GET /healthz`)
    - [ ] API Docs (`GET /_docs`)

#### 2.8. Error Responses
- [x] Research and document:
    - [x] Standard S3 error XML responses for all endpoints (in progress, see docs/examples/)

#### 2.9. OpenAPI Annotations
- [ ] Annotate all controllers and models with OpenAPI doc comments for self-documentation.

---

### Research Progress Summary

**Completed:**
- Presigned GET/PUT Object (docs/examples, request/response, error XML)
- ACLs & CORS (docs/examples, config, error XML)
- Healthcheck endpoint (docs)
- Standard S3 error XML responses (for completed endpoints)

**Still to be researched/documented:**
- API Docs endpoint (`/_docs`)
- OpenAPI annotations for all controllers/models

See `/docs/` for detailed documentation and `/docs/examples/` for error XML examples.

---

### 3. Config Management
- [ ] Define YAML config schema.
- [ ] Implement config loader with hot-reloading.
- [ ] **Validate config on load and reload** (required fields, types, value ranges, etc.)

---

### 4. Logging
- [ ] Integrate logging framework.
- [ ] Expose config options for logging.

---

### 5. TLS & Server Setup
- [ ] Implement HTTP and optional HTTPS server.
- [ ] Integrate Let's Encrypt (DigitalOcean DNS only).
- [ ] Add healthcheck endpoint.

---

### 6. Authentication & Authorization
- [ ] Implement AWSv4 signature verification.
- [ ] Enforce IAM-like permissions for credentials.
- [ ] Enforce bucket ACLs (public, IP, CORS).

---

### 7. Bucket Operations

#### 7.1. Create Bucket
- [ ] Implement `PUT /{bucket}`.
- [ ] **Validate**: Bucket name, existence, permissions.

#### 7.2. List Buckets
- [ ] Implement `GET /`.
- [ ] **Validate**: Permissions.

#### 7.3. Delete Bucket
- [ ] Implement `DELETE /{bucket}`.
- [ ] **Validate**: Bucket existence, emptiness, permissions.

#### 7.4. List Objects in Bucket
- [ ] Implement `GET /{bucket}`.
- [ ] **Validate**: Bucket existence, permissions, query params.

#### 7.5. List Objects V2
- [ ] Implement `GET /{bucket}?list-type=2`.
- [ ] **Validate**: Bucket existence, permissions, query params (prefix, delimiter, continuation-token, etc.).

---

### 8. Object Operations

#### 8.1. Put Object
- [ ] Implement `PUT /{bucket}/{object}`.
- [ ] **Validate**: Bucket existence, object name, permissions, content headers.

#### 8.2. Get Object
- [ ] Implement `GET /{bucket}/{object}`.
- [ ] **Validate**: Bucket/object existence, permissions.

#### 8.3. Get Object (Byte Range)
- [ ] Implement `GET /{bucket}/{object}` with `Range` header.
- [ ] **Validate**: Range header, object existence, permissions.

#### 8.4. Delete Object
- [ ] Implement `DELETE /{bucket}/{object}`.
- [ ] **Validate**: Bucket/object existence, permissions.

---

### 9. Multipart Uploads

#### 9.1. Initiate Multipart Upload
- [ ] Implement `POST /{bucket}/{object}?uploads`.
- [ ] **Validate**: Bucket existence, permissions.

#### 9.2. Upload Part
- [ ] Implement `PUT /{bucket}/{object}?partNumber={PartNumber}&uploadId={UploadId}`.
- [ ] **Validate**: UploadId, part number, permissions.

#### 9.3. Complete Multipart Upload
- [ ] Implement `POST /{bucket}/{object}?uploadId={UploadId}`.
- [ ] **Validate**: UploadId, parts, permissions.

#### 9.4. Abort Multipart Upload
- [ ] Implement `DELETE /{bucket}/{object}?uploadId={UploadId}`.
- [ ] **Validate**: UploadId, permissions.

#### 9.5. List Multipart Uploads
- [ ] Implement `GET /{bucket}?uploads`.
- [ ] **Validate**: Bucket existence, permissions.

#### 9.6. List Parts
- [ ] Implement `GET /{bucket}/{object}?uploadId={UploadId}`.
- [ ] **Validate**: UploadId, permissions.

#### 9.7. Multipart Expiry
- [ ] Implement periodic cleanup of expired multipart uploads.

---

### 10. Presigned URLs

#### 10.1. Presigned GET Object
- [ ] Implement presigned GET logic.
- [ ] **Validate**: Signature, expiry, permissions, IP (if restricted).

#### 10.2. Presigned PUT Object
- [ ] Implement presigned PUT logic.
- [ ] **Validate**: Signature, expiry, permissions, IP (if restricted).

---

### 11. CORS Support

#### 11.1. CORS Preflight
- [ ] Implement `OPTIONS /{bucket}/{object}`.
- [ ] **Validate**: CORS rules for bucket.

#### 11.2. CORS Headers
- [ ] Add CORS headers to relevant responses.

---

### 12. Testing & Compatibility

#### 12.1. Unit Tests
- [ ] Add unit tests for all components.

#### 12.2. Integration Tests
- [ ] Add integration tests for all endpoints (clean up after test).
- [ ] Test with AWS CLI and s3cmd for compatibility.

---

### 13. API Documentation

#### 13.1. Serve OpenAPI/Swagger Docs
- [ ] Serve docs at `/_docs`.

#### 13.2. Ensure Coverage
- [ ] Ensure all endpoints, request/response models, and error codes are documented.

---

## Example Config File

```yaml
# Storage configuration: where to store buckets and objects
storage:
  location: "/var/lib/s3-clone"

# Default region for new buckets (if not specified in request)
region:
  default: "de-muc-01"

# Logging configuration: set levels per component (server, storage, auth, etc.)
logging:
  format: "json"  # or "text"
  levels:
    server: "info"
    storage: "warn"
    auth: "debug"

# Server configuration
server:
  http:
    enabled: true
    port: 9000
  https:
    enabled: true
    port: 9443
    letsencrypt:
      enabled: true
      email: "admin@example.com"
      domains: ["s3.local"]
      do_token: "DO_API_TOKEN"

# Credentials: IAM-like permissions
credentials:
  - access_key: "AKIA..."
    secret_key: "SECRET..."
    permissions:
      - action: "Create*"
        resource: "*"
      - action: "DeleteObject"
        resource: "private-bucket/*"

# Bucket ACLs: not linked to credentials
bucket_acls:
  - bucket: "public-bucket"
    public: true
    cors:
      allowed_origins: ["*"]
      allowed_methods: ["GET", "PUT"]
  - bucket: "private-bucket"
    allowed_ips: ["192.168.1.0/24"]

# Multipart upload settings
multipart:
  expiry_seconds: 86400  # 24 hours

# Config reload triggers
config_reload:
  sighup: true
  api: true
  fsevents: true
```

---

## Bucket and Multipart Metadata Storage

### Bucket Metadata
- Each bucket contains a metadata file at `_metadata/bucket.yaml` (relative to the bucket root).
- This file is written at bucket creation and is read-only from a process perspective.
- It stores:
  - `name`: Bucket name
  - `region`: Region (from config or request)
  - `created`: Creation timestamp (ISO8601)
  - `created_by`: Creator (user/credential)
  - `acls`: Bucket ACLs (public, allowed_ips, etc.)
  - `cors`: CORS rules (allowed origins, methods, etc.)
- If ACLs or CORS are not specified, defaults from config are used.
- **Object metadata is not stored here.**

#### Example: `_metadata/bucket.yaml`
```yaml
name: example-bucket
region: de-muc-01
created: "2024-06-12T10:00:00Z"
created_by: jan-niklas.kohlhaas
acls:
  public: false
  allowed_ips: []
cors:
  allowed_origins: ["*"]
  allowed_methods: ["GET", "PUT"]
```

### Multipart Upload Metadata
- Each multipart upload has its own metadata file at `_metadata/multipart/<multipart-id>.yaml` (relative to the bucket root).
- This file is created when a multipart upload is initiated and updated as parts are uploaded.
- It stores:
  - `key`: Object key being uploaded
  - `upload_id`: Multipart upload ID
  - `initiated`: Initiation timestamp (ISO8601)
  - `initiated_by`: Creator (user/credential)
  - `parts`: List of uploaded parts (number, etag, size, last_modified)

#### Example: `_metadata/multipart/abc123.yaml`
```yaml
key: videos/bigfile.mp4
upload_id: abc123
initiated: "2024-06-12T10:05:00Z"
initiated_by: jan-niklas.kohlhaas
parts:
  - part_number: 1
    etag: "part1etag"
    size: 5242880
    last_modified: "2024-06-12T10:06:00Z"
  - part_number: 2
    etag: "part2etag"
    size: 5242880
    last_modified: "2024-06-12T10:07:00Z"
```

--- 

## Workflow

1. **Edit Rust code and OpenAPI annotations** for any API/model changes.
2. **Controllers and models are self-documenting**; keep docs and code in sync.
3. **Implement/extend logic** in the server code.
4. **Update tests and docs** as needed.

---

## Changelogs

- **2024-06-10**: Initial README created with detailed implementation steps, config example, and changelogs section.
- **2024-06-11**: Project initialized with cargo, dependencies added, and directory structure created (`src/api`, `src/auth`, `src/config`, `src/docs`, `src/logging`, `src/storage`).
