# S3 Abort Multipart Upload (`DELETE /{bucket}/{object}?uploadId={UploadId}`)

## Overview
Aborts a multipart upload, discarding all uploaded parts. Requires authentication. Used to clean up incomplete uploads.

---

## Request

### HTTP Method & Path
```
DELETE /{bucket}/{object}?uploadId={UploadId} HTTP/1.1
Host: localhost:9000
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- (No standard optional headers for Abort Multipart Upload, but any future S3 extensions or custom headers will be listed here if needed)

---

## Response

### Success
- **Status:** `204 No Content`
- **Headers:**
  - `Content-Length: 0`
- **Body:** Empty

### Error Cases
- **404 NoSuchBucket**: The specified bucket does not exist.
- **404 NoSuchUpload**: The specified upload ID does not exist.
- **403 AccessDenied**: Not authorized.

---

## Validation
- Only authenticated requests allowed
- Bucket and upload ID must exist

---

## Example Error Responses
See `docs/examples/multipart_abort/` for XML examples.

---

## Notes
- Uses path-style URL: `DELETE /{bucket}/{object}?uploadId={UploadId}` on `localhost`
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 AbortMultipartUpload API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_AbortMultipartUpload.html)
