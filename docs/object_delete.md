# S3 Delete Object (`DELETE /{bucket}/{object}`)

## Overview
Deletes the specified object from the bucket. Succeeds even if the object does not exist (idempotent). Requires authentication.

---

## Request

### HTTP Method & Path
```
DELETE /{bucket}/{object} HTTP/1.1
Host: localhost:9000
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- (No standard optional headers for Delete Object, but any future S3 extensions or custom headers will be listed here if needed)

---

## Response

### Success
- **Status:** `204 No Content`
- **Headers:**
  - `Content-Length: 0`
- **Body:** Empty

### Error Cases
- **404 NoSuchBucket**: The specified bucket does not exist.
- **403 AccessDenied**: Not authorized.

---

## Validation
- Only authenticated requests allowed
- Bucket must exist
- Object key must be valid (see S3 object key rules)

---

## Example Error Responses
See `docs/examples/object_delete/` for XML examples.

---

## Notes
- Uses path-style URL: `DELETE /{bucket}/{object}` on `localhost`
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 DeleteObject API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObject.html)
