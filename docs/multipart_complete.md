# S3 Complete Multipart Upload (`POST /{bucket}/{object}?uploadId={UploadId}`)

## Overview
Completes a multipart upload by assembling previously uploaded parts. Requires authentication. Returns the final object's ETag and location.

---

## Request

### HTTP Method & Path
```
POST /{bucket}/{object}?uploadId={UploadId} HTTP/1.1
Host: localhost:9000
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`
- `Content-Length`: Size of the XML body

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- (No standard optional headers for Complete Multipart Upload, but any future S3 extensions or custom headers will be listed here if needed)

### Request Body (XML)
A list of parts to assemble, in order:
```xml
<CompleteMultipartUpload>
  <Part>
    <PartNumber>1</PartNumber>
    <ETag>"etag-part-1"</ETag>
  </Part>
  <Part>
    <PartNumber>2</PartNumber>
    <ETag>"etag-part-2"</ETag>
  </Part>
  <!-- ... -->
</CompleteMultipartUpload>
```

---

## Response

### Success
- **Status:** `200 OK`
- **Headers:**
  - `Content-Type: application/xml`
- **Body:** XML with final object info

#### Example
```xml
<CompleteMultipartUploadResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Location>http://localhost:9000/example-bucket/example-object.txt</Location>
  <Bucket>example-bucket</Bucket>
  <Key>example-object.txt</Key>
  <ETag>"final-etag-value"</ETag>
</CompleteMultipartUploadResult>
```

### Error Cases
- **404 NoSuchBucket**: The specified bucket does not exist.
- **404 NoSuchUpload**: The specified upload ID does not exist.
- **400 InvalidPart**: The part is invalid or missing.
- **400 InvalidPartOrder**: The list of parts is not in ascending order.
- **403 AccessDenied**: Not authorized.

---

## Validation
- Only authenticated requests allowed
- Bucket and upload ID must exist
- Parts must be valid and in ascending order
- All parts must be present

---

## Example Error Responses
See `docs/examples/multipart_complete/` for XML examples.

---

## Notes
- Uses path-style URL: `POST /{bucket}/{object}?uploadId={UploadId}` on `localhost`
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 CompleteMultipartUpload API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_CompleteMultipartUpload.html)
