# S3 Get Object (Byte Range) (`GET /{bucket}/{object}` with Range header)

## Overview
Retrieves a byte range of an object from the specified bucket. Requires authentication. Uses the `Range` header to specify the byte range.

---

## Request

### HTTP Method & Path
```
GET /{bucket}/{object} HTTP/1.1
Host: localhost:9000
Range: bytes=0-99
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`
- `Range`: The byte range to retrieve (e.g., `bytes=0-99`)

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- `If-Modified-Since`, `If-Unmodified-Since`, `If-Match`, `If-None-Match`: Conditional requests
- (Other S3 headers as needed for future features)

---

## Response

### Success
- **Status:** `206 Partial Content`
- **Headers:**
  - `Content-Type`: MIME type of the object
  - `Content-Range`: The range returned (e.g., `bytes 0-99/1234`)
  - `Content-Length`: Size of the returned range in bytes
  - `ETag`: The entity tag for the object (usually the MD5 hash)
- **Body:** The requested byte range of the object

#### Example
```
HTTP/1.1 206 Partial Content
Content-Type: text/plain
Content-Range: bytes 0-99/1234
Content-Length: 100
ETag: "d41d8cd98f00b204e9800998ecf8427e"

<partial file contents>
```

### Error Cases
- **404 NoSuchBucket**: The specified bucket does not exist.
- **404 NoSuchKey**: The specified object does not exist.
- **416 InvalidRange**: The specified range is not satisfiable.
- **403 AccessDenied**: Not authorized.

---

## Validation
- Only authenticated requests allowed
- Bucket and object must exist
- Object key must be valid (see S3 object key rules)
- Range header must be valid and satisfiable

---

## Example Error Responses
See `docs/examples/object_get_range/` for XML examples.

---

## Notes
- Uses path-style URL: `GET /{bucket}/{object}` on `localhost` with `Range` header
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 GetObject API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html)
