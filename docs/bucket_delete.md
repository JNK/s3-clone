# S3 Delete Bucket (`DELETE /{bucket}`)

## Overview
Deletes the specified bucket. The bucket must be empty. Requires authentication. Bucket name must be globally unique.

---

## Request

### HTTP Method & Path
```
DELETE /{bucket} HTTP/1.1
Host: localhost:9000
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- (No standard optional headers for Delete Bucket, but any future S3 extensions or custom headers will be listed here if needed)

---

## Response

### Success
- **Status:** `204 No Content`
- **Headers:**
  - `Content-Length: 0`
- **Body:** Empty

### Error Cases
- **404 NoSuchBucket**: The specified bucket does not exist.
- **409 BucketNotEmpty**: The bucket is not empty.
- **403 AccessDenied**: Not authorized.

---

## Validation
- Only authenticated requests allowed
- Bucket must exist
- Bucket must be empty
- No request body or query parameters are used

---

## Example Error Responses
See `docs/examples/bucket_delete/` for XML examples (to be created).

---

## Notes
- Uses path-style URL: `DELETE /{bucket}` on `localhost`
- Default region is `de-muc-01` (configurable via config), but region is not relevant for this operation
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 DeleteBucket API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucket.html)
