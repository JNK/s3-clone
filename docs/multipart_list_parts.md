# S3 List Parts (`GET /{bucket}/{object}?uploadId={UploadId}`)

## Overview
Lists the parts that have been uploaded for a specific multipart upload. Requires authentication. Used to resume or complete multipart uploads.

---

## Request

### HTTP Method & Path
```
GET /{bucket}/{object}?uploadId={UploadId} HTTP/1.1
Host: localhost:9000
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- (No standard optional headers for List Parts, but any future S3 extensions or custom headers will be listed here if needed)

### Query Parameters
- `uploadId` (required): The upload ID whose parts are to be listed
- `max-parts` (optional): Sets the maximum number of parts returned in the response (default: 1000)
- `part-number-marker` (optional): Specifies the part number to start with when listing parts

---

## Response

### Success
- **Status:** `200 OK`
- **Headers:**
  - `Content-Type: application/xml`
- **Body:** XML listing parts

#### Example
```xml
<ListPartsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Bucket>example-bucket</Bucket>
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
  <PartNumberMarker>0</PartNumberMarker>
  <NextPartNumberMarker>2</NextPartNumberMarker>
  <MaxParts>1000</MaxParts>
  <IsTruncated>false</IsTruncated>
  <Part>
    <PartNumber>1</PartNumber>
    <LastModified>2024-06-11T12:00:00.000Z</LastModified>
    <ETag>"etag-part-1"</ETag>
    <Size>5242880</Size>
  </Part>
  <Part>
    <PartNumber>2</PartNumber>
    <LastModified>2024-06-11T13:00:00.000Z</LastModified>
    <ETag>"etag-part-2"</ETag>
    <Size>5242880</Size>
  </Part>
  <!-- ... -->
</ListPartsResult>
```

### Error Cases
- **404 NoSuchBucket**: The specified bucket does not exist.
- **404 NoSuchUpload**: The specified upload ID does not exist.
- **403 AccessDenied**: Not authorized.

---

## Validation
- Only authenticated requests allowed
- Bucket and upload ID must exist
- Query parameters must be valid

---

## Example Error Responses
See `docs/examples/multipart_list_parts/` for XML examples.

---

## Notes
- Uses path-style URL: `GET /{bucket}/{object}?uploadId={UploadId}` on `localhost`
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 ListParts API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html)
