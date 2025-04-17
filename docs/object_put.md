# S3 Put Object (`PUT /{bucket}/{object}`)

## Overview
Uploads an object to the specified bucket. Overwrites the object if it already exists. Requires authentication.

---

## Request

### HTTP Method & Path
```
PUT /{bucket}/{object} HTTP/1.1
Host: localhost:9000
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`
- `Content-Length`: Size of the object in bytes

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- `Content-Type`: MIME type of the object
- `x-amz-meta-*`: User-defined metadata
- `x-amz-storage-class`: Storage class
- `x-amz-acl`: ACL for the object
- `x-amz-server-side-encryption`: Encryption settings
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

#### Example
```
HTTP/1.1 200 OK
ETag: "d41d8cd98f00b204e9800998ecf8427e"
Content-Length: 0
```

### Error Cases
- **404 NoSuchBucket**: The specified bucket does not exist.
- **400 InvalidObjectName**: The object key is invalid.
- **403 AccessDenied**: Not authorized.

---

## Validation
- Only authenticated requests allowed
- Bucket must exist
- Object key must be valid (see S3 object key rules)
- Content-Length must match the body size

---

## Example Error Responses
See `docs/examples/object_put/` for XML examples.

---

## Notes
- Uses path-style URL: `PUT /{bucket}/{object}` on `localhost`
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 PutObject API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html)
