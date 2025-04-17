# S3 List Objects V2 (`GET /{bucket}?list-type=2`)

## Overview
Lists objects in the specified bucket using the V2 API. Supports filtering and pagination with continuation tokens. Returns an XML response with object metadata. Requires authentication.

---

## Request

### HTTP Method & Path
```
GET /{bucket}?list-type=2 HTTP/1.1
Host: localhost:9000
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- (No standard optional headers for List Objects V2, but any future S3 extensions or custom headers will be listed here if needed)

### Query Parameters
- `list-type=2` (required): Indicates V2 API
- `prefix` (optional): Limits the response to keys that begin with the specified prefix.
- `delimiter` (optional): Groups keys that contain the same string between the prefix and the first occurrence of the delimiter.
- `start-after` (optional): Specifies the key to start with when listing objects in a bucket.
- `continuation-token` (optional): Used for paginated results.
- `max-keys` (optional): Sets the maximum number of keys returned in the response (default: 1000).

---

## Response

### Success
- **Status:** `200 OK`
- **Headers:**
  - `Content-Type: application/xml`
- **Body:** XML listing objects and common prefixes

#### Example
```xml
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Name>example-bucket</Name>
  <Prefix></Prefix>
  <KeyCount>2</KeyCount>
  <MaxKeys>1000</MaxKeys>
  <Delimiter></Delimiter>
  <IsTruncated>false</IsTruncated>
  <Contents>
    <Key>object1.txt</Key>
    <LastModified>2024-06-11T12:00:00.000Z</LastModified>
    <ETag>"etag-value"</ETag>
    <Size>123</Size>
    <StorageClass>STANDARD</StorageClass>
  </Contents>
  <Contents>
    <Key>object2.txt</Key>
    <LastModified>2024-06-11T13:00:00.000Z</LastModified>
    <ETag>"etag-value-2"</ETag>
    <Size>456</Size>
    <StorageClass>STANDARD</StorageClass>
  </Contents>
  <!-- If truncated, include NextContinuationToken -->
  <!-- <NextContinuationToken>token-value</NextContinuationToken> -->
</ListBucketResult>
```

### Error Cases
- **404 NoSuchBucket**: The specified bucket does not exist.
- **403 AccessDenied**: Not authorized.

---

## Validation
- Only authenticated requests allowed
- Bucket must exist
- Query parameters must be valid (e.g., max-keys is a positive integer)
- If `continuation-token` is provided, it must be valid

---

## Example Error Responses
See `docs/examples/bucket_list_objects_v2/` for XML examples (to be created).

---

## Notes
- Uses path-style URL: `GET /{bucket}?list-type=2` on `localhost`
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 ListObjectsV2 API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html)
