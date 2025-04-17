# S3 List Multipart Uploads (`GET /{bucket}?uploads`)

## Overview
Lists in-progress multipart uploads for the specified bucket. Requires authentication. Supports filtering by prefix, delimiter, key-marker, and upload-id-marker.

---

## Request

### HTTP Method & Path
```
GET /{bucket}?uploads HTTP/1.1
Host: localhost:9000
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- (No standard optional headers for List Multipart Uploads, but any future S3 extensions or custom headers will be listed here if needed)

### Query Parameters
- `uploads` (required): Indicates this is a list multipart uploads request
- `prefix` (optional): Limits the response to keys that begin with the specified prefix
- `delimiter` (optional): Groups keys that contain the same string between the prefix and the first occurrence of the delimiter
- `key-marker` (optional): Specifies the key to start with when listing uploads
- `upload-id-marker` (optional): Specifies the upload ID to start with when listing uploads
- `max-uploads` (optional): Sets the maximum number of uploads returned in the response (default: 1000)

---

## Response

### Success
- **Status:** `200 OK`
- **Headers:**
  - `Content-Type: application/xml`
- **Body:** XML listing multipart uploads

#### Example
```xml
<ListMultipartUploadsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Bucket>example-bucket</Bucket>
  <KeyMarker></KeyMarker>
  <UploadIdMarker></UploadIdMarker>
  <NextKeyMarker></NextKeyMarker>
  <NextUploadIdMarker></NextUploadIdMarker>
  <MaxUploads>1000</MaxUploads>
  <IsTruncated>false</IsTruncated>
  <Upload>
    <Key>example-object.txt</Key>
    <UploadId>example-upload-id</UploadId>
    <Initiator>
      <ID>initiator-id</ID>
      <DisplayName>initiator</DisplayName>
    </Initiator>
    <Owner>
      <ID>owner-id</ID>
      <DisplayName>owner</DisplayName>
    </Owner>
    <StorageClass>STANDARD</StorageClass>
    <Initiated>2024-06-11T12:00:00.000Z</Initiated>
  </Upload>
  <!-- ... -->
</ListMultipartUploadsResult>
```

### Error Cases
- **404 NoSuchBucket**: The specified bucket does not exist.
- **403 AccessDenied**: Not authorized.

---

## Validation
- Only authenticated requests allowed
- Bucket must exist
- Query parameters must be valid

---

## Example Error Responses
See `docs/examples/multipart_list_uploads/` for XML examples.

---

## Notes
- Uses path-style URL: `GET /{bucket}?uploads` on `localhost`
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 ListMultipartUploads API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListMultipartUploads.html)
