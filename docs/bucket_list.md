# S3 List Buckets (`GET /`)

## Overview
Lists all buckets owned by the authenticated user. Requires authentication. Returns an XML response with all bucket names and their creation dates.

---

## Request

### HTTP Method & Path
```
GET / HTTP/1.1
Host: localhost:9000
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`

### Optional Headers (Deprioritized)
These headers are deprioritized and will be ignored unless related to a feature being built:
- (No standard optional headers for List Buckets, but any future S3 extensions or custom headers will be listed here if needed)

---

## Response

### Success
- **Status:** `200 OK`
- **Headers:**
  - `Content-Type: application/xml`
- **Body:** XML listing all buckets

#### Example
```xml
<ListAllMyBucketsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Owner>
    <ID>owner-id</ID>
    <DisplayName>owner-display-name</DisplayName>
  </Owner>
  <Buckets>
    <Bucket>
      <Name>bucket1</Name>
      <CreationDate>2024-06-11T12:00:00.000Z</CreationDate>
    </Bucket>
    <Bucket>
      <Name>bucket2</Name>
      <CreationDate>2024-06-11T13:00:00.000Z</CreationDate>
    </Bucket>
  </Buckets>
</ListAllMyBucketsResult>
```

### Error Cases
- **403 AccessDenied**: Not authorized.

---

## Validation
- Only authenticated requests allowed
- No request body or query parameters are used

---

## Notes
- Uses path-style URL: `GET /` on `localhost`
- Default region is `de-muc-01` (configurable via config), but region is not relevant for this operation
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 ListBuckets API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBuckets.html)
