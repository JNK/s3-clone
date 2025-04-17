# S3 Upload Part (`PUT /{bucket}/{object}?partNumber={PartNumber}&uploadId={UploadId}`)

## Overview
Uploads a part in a multipart upload. Requires authentication. Each part (except the last) must be at least 5 MB.

---

## Request

### HTTP Method & Path
```
PUT /{bucket}/{object}?partNumber={PartNumber}&uploadId={UploadId} HTTP/1.1
Host: localhost:9000
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`
- `Content-Length`: Size of the part in bytes

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- `Content-MD5`: Base64-encoded 128-bit MD5 digest of the part data
- (Other S3 headers as needed for future features)

### Request Body
- The part data (binary or text)

---

## Response

### Success
- **Status:** `200 OK`
- **Headers:**
  - `ETag`: The entity tag for the uploaded part (usually the MD5 hash)
- **Body:** Empty

#### Example
```
HTTP/1.1 200 OK
ETag: "d41d8cd98f00b204e9800998ecf8427e"
Content-Length: 0
```

### Error Cases
- **404 NoSuchBucket**: The specified bucket does not exist.
- **404 NoSuchUpload**: The specified upload ID does not exist.
- **400 InvalidPart**: The part is invalid (e.g., too small, missing part number).
- **403 AccessDenied**: Not authorized.

---

## Validation
- Only authenticated requests allowed
- Bucket and upload ID must exist
- Part number must be valid (1-10000)
- Part size must be at least 5 MB (except last part)

---

## Example Error Responses
See `docs/examples/multipart_upload_part/` for XML examples.

---

## Notes
- Uses path-style URL: `PUT /{bucket}/{object}?partNumber={PartNumber}&uploadId={UploadId}` on `localhost`
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 UploadPart API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html)
