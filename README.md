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
- [x] Define YAML config schema.
- [x] Implement config loader with hot-reloading.
- [x] **Validate config on load and reload** (required fields, types, value ranges, etc.)

---

### 4. Logging

This project uses [`env_logger`](https://docs.rs/env_logger) and the standard [`log`](https://docs.rs/log) crate for logging.

- **Log level is set via the `RUST_LOG` environment variable** (e.g., `RUST_LOG=debug ./s3-clone`).
- If `RUST_LOG` is not set, the default log level is `info`.
- Log output is plain text to the console.
- Example usage in code:
  ```rust
  use log::{info, warn, error, debug};
  info!("Server started");
  warn!("Low disk space");
  error!("Something went wrong: {}", "details");
  debug!("Debug info: {:?}", (1, 2, 3));
  ```
- **No per-module log levels or JSON output are supported.**
- **No logging configuration is required in `config.yaml`**; just set `RUST_LOG` as needed.

---

### 5. TLS & Server Setup
- [x] Implement HTTP server.
- [ ] Implement HTTPS server.
- [ ] Integrate Let's Encrypt (DigitalOcean DNS only).
- [x] Add healthcheck endpoint.

---

### 6. Bucket Operations

#### 6.1. Create Bucket
- [ ] Implement `PUT /{bucket}`.
- [ ] **Validate**: Bucket name, existence, permissions.

#### 6.2. List Buckets
- [ ] Implement `GET /`.
- [ ] **Validate**: Permissions.

#### 6.3. Delete Bucket
- [ ] Implement `DELETE /{bucket}`.
- [ ] **Validate**: Bucket existence, emptiness, permissions.

#### 6.4. List Objects in Bucket
- [ ] Implement `GET /{bucket}`.
- [ ] **Validate**: Bucket existence, permissions, query params.

#### 6.5. List Objects V2
- [ ] Implement `GET /{bucket}?list-type=2`.
- [ ] **Validate**: Bucket existence, permissions, query params (prefix, delimiter, continuation-token, etc.).

---

### 7. Object Operations

#### 7.1. Put Object
- [ ] Implement `PUT /{bucket}/{object}`.
- [ ] **Validate**: Bucket existence, object name, permissions, content headers.

#### 7.2. Get Object
- [ ] Implement `GET /{bucket}/{object}`.
- [ ] **Validate**: Bucket/object existence, permissions.

#### 7.3. Get Object (Byte Range)
- [ ] Implement `GET /{bucket}/{object}` with `Range` header.
- [ ] **Validate**: Range header, object existence, permissions.

#### 7.4. Delete Object
- [ ] Implement `DELETE /{bucket}/{object}`.
- [ ] **Validate**: Bucket/object existence, permissions.

---

### 8. Multipart Uploads

#### 8.1. Initiate Multipart Upload
- [ ] Implement `POST /{bucket}/{object}?uploads`.
- [ ] **Validate**: Bucket existence, permissions.

#### 8.2. Upload Part
- [ ] Implement `PUT /{bucket}/{object}?partNumber={PartNumber}&uploadId={UploadId}`.
- [ ] **Validate**: UploadId, part number, permissions.

#### 8.3. Complete Multipart Upload
- [ ] Implement `POST /{bucket}/{object}?uploadId={UploadId}`.
- [ ] **Validate**: UploadId, parts, permissions.

#### 8.4. Abort Multipart Upload
- [ ] Implement `DELETE /{bucket}/{object}?uploadId={UploadId}`.
- [ ] **Validate**: UploadId, permissions.

#### 8.5. List Multipart Uploads
- [ ] Implement `GET /{bucket}?uploads`.
- [ ] **Validate**: Bucket existence, permissions.

#### 8.6. List Parts
- [ ] Implement `GET /{bucket}/{object}?uploadId={UploadId}`.
- [ ] **Validate**: UploadId, permissions.

#### 8.7. Multipart Expiry
- [ ] Implement periodic cleanup of expired multipart uploads.

---

### 9. Authentication & Authorization
- Implement AWSv4 signature verification.
- Enforce IAM-like permissions for credentials.
- Enforce bucket ACLs (public, IP, CORS).

---

### 10. Presigned URLs

#### 10.1. Presigned GET Object
- [ ] Implement presigned GET logic.
- [ ] **Validate**: Signature, expiry, permissions, IP (if restricted).

#### 10.2. Presigned PUT Object
- [ ] Implement presigned PUT logic.
- [ ] **Validate**: Signature, expiry, permissions, IP (if restricted).

### 11. CORS Support

#### 11.3. CORS Preflight
- [ ] Implement `OPTIONS /{bucket}/{object}`.
- [ ] **Validate**: CORS rules for bucket.

#### 11.4. CORS Headers
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

# Server configuration
server:
  http:
    enabled: true
    port: 9000
    hhost: 0.0.0.0
  https:
    enabled: true
    port: 9443
    letsencrypt:
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
default_acls:
  public: false
  allowed_ips: []

default_cors:
  allowed_origins: ["*"]
  allowed_methods: ["GET", "PUT"]

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

## Workflow

1. **Edit Rust code and OpenAPI annotations** for any API/model changes.
2. **Controllers and models are self-documenting**; keep docs and code in sync.
3. **Implement/extend logic** in the server code.
4. **Update tests and docs** as needed.

---

## Changelogs

- **2024-06-10**: Initial README created with detailed implementation steps, config example, and changelogs section.
- **2024-06-11**: Project initialized with cargo, dependencies added, and directory structure created (`src/api`, `src/auth`, `src/config`, `src/docs`, `src/logging`, `src/storage`).
