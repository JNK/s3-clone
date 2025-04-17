# S3 Initiate Multipart Upload (`POST /{bucket}/{object}?uploads`)

## Overview
Initiates a multipart upload and returns an upload ID. Requires authentication. Used for uploading large objects in parts.

---

## Request

### HTTP Method & Path
```
POST /{bucket}/{object}?uploads HTTP/1.1
Host: localhost:9000
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- `x-amz-meta-*`: User-defined metadata
- `x-amz-storage-class`: Storage class
- `x-amz-acl`: ACL for the object
- (Other S3 headers as needed for future features)

---

## Response

### Success
- **Status:** `200 OK`
- **Headers:**
  - `Content-Type: application/xml`
- **Body:** XML with upload ID

#### Example
```xml
<InitiateMultipartUploadResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Bucket>example-bucket</Bucket>
  <Key>example-object.txt</Key>
  <UploadId>example-upload-id</UploadId>
</InitiateMultipartUploadResult>
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

---

## Example Error Responses
See `docs/examples/multipart_initiate/` for XML examples.

---

## Notes
- Uses path-style URL: `POST /{bucket}/{object}?uploads` on `localhost`
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 CreateMultipartUpload API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html)
