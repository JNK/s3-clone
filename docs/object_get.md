# S3 Get Object (`GET /{bucket}/{object}`)

## Overview
Retrieves an object from the specified bucket. Requires authentication. Supports byte-range requests (see separate doc for details).

---

## Request

### HTTP Method & Path
```
GET /{bucket}/{object} HTTP/1.1
Host: localhost:9000
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- `Range`: For byte-range requests (see separate doc)
- `If-Modified-Since`, `If-Unmodified-Since`, `If-Match`, `If-None-Match`: Conditional requests
- (Other S3 headers as needed for future features)

---

## Response

### Success
- **Status:** `200 OK`
- **Headers:**
  - `Content-Type`: MIME type of the object
  - `Content-Length`: Size of the object in bytes
  - `ETag`: The entity tag for the object (usually the MD5 hash)
- **Body:** The object data (binary or text)

#### Example
```
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 123
ETag: "d41d8cd98f00b204e9800998ecf8427e"

<file contents>
```

### Error Cases
- **404 NoSuchBucket**: The specified bucket does not exist.
- **404 NoSuchKey**: The specified object does not exist.
- **403 AccessDenied**: Not authorized.

---

## Validation
- Only authenticated requests allowed
- Bucket and object must exist
- Object key must be valid (see S3 object key rules)

---

## Example Error Responses
See `docs/examples/object_get/` for XML examples.

---

## Notes
- Uses path-style URL: `GET /{bucket}/{object}` on `localhost`
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 GetObject API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html)
