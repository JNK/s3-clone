# S3 Presigned PUT Object (`PUT /{bucket}/{object}?X-Amz-...`)

## Overview
Uploads an object using a presigned URL. No authentication header required; access is granted if the signature, expiry, and (optionally) IP match. Overwrites the object if it already exists.

---

## Request

### HTTP Method & Path
```
PUT /{bucket}/{object}?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=...&X-Amz-Date=...&X-Amz-Expires=...&X-Amz-SignedHeaders=...&X-Amz-Signature=... [other params] HTTP/1.1
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

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- `Content-Type`: MIME type of the object
- (Other S3 headers as needed for future features)

### Request Body
- The object data (binary or text)

---

## Response

### Success
- **Status:** `200 OK`
- **Headers:**
  - `ETag`: The entity tag for the object (usually the MD5 hash)
- **Body:** Empty

### Error Cases
- **403 AccessDenied**: Signature invalid, expired, or IP mismatch.
- **404 NoSuchBucket**: The specified bucket does not exist.
- **400 InvalidObjectName**: The object key is invalid.

---

## Validation
- Signature must be valid
- URL must not be expired
- If IP restriction is present, must match client IP
- Bucket must exist
- Object key must be valid (see S3 object key rules)

---

## Example Error Responses
See `docs/examples/presigned_put/` for XML examples (to be created).

---

## Notes
- Uses path-style URL: `PUT /{bucket}/{object}?X-Amz-...` on `localhost`
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 Presigned URLs](https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html)
