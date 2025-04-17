# S3 Presigned GET Object (`GET /{bucket}/{object}?X-Amz-...`)

## Overview
Retrieves an object using a presigned URL. No authentication header required; access is granted if the signature, expiry, and (optionally) IP match. Supports byte-range requests.

---

## Request

### HTTP Method & Path
```
GET /{bucket}/{object}?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=...&X-Amz-Date=...&X-Amz-Expires=...&X-Amz-SignedHeaders=...&X-Amz-Signature=... [other params] HTTP/1.1
Host: localhost:9000
```

### Required Query Parameters
- `X-Amz-Algorithm`: Always `AWS4-HMAC-SHA256`
- `X-Amz-Credential`: Credential string
- `X-Amz-Date`: Date in ISO8601 format
- `X-Amz-Expires`: Expiry in seconds
- `X-Amz-SignedHeaders`: Signed headers
- `X-Amz-Signature`: Signature
- (Other AWS S3 presign params as needed)

### Optional Query Parameters
- `X-Amz-Security-Token`: For temporary credentials
- `X-Amz-Expires`: Expiry in seconds (default: 3600, max: configurable)
- `X-Amz-Source-IP`: Restrict to a specific IP (if supported)

### Optional Headers
- `Range`: For byte-range requests
- (Other S3 headers as needed for future features)

---

## Response

### Success
- **Status:** `200 OK` (or `206 Partial Content` for range requests)
- **Headers:**
  - `Content-Type`: MIME type of the object
  - `Content-Length`: Size of the object in bytes
  - `ETag`: The entity tag for the object (usually the MD5 hash)
- **Body:** The object data (binary or text)

### Error Cases
- **403 AccessDenied**: Signature invalid, expired, or IP mismatch.
- **404 NoSuchBucket**: The specified bucket does not exist.
- **404 NoSuchKey**: The specified object does not exist.

---

## Validation
- Signature must be valid
- URL must not be expired
- If IP restriction is present, must match client IP
- Bucket and object must exist

---

## Example Error Responses
See `docs/examples/presigned_get/` for XML examples (to be created).

---

## Notes
- Uses path-style URL: `GET /{bucket}/{object}?X-Amz-...` on `localhost`
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 Presigned URLs](https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html)
