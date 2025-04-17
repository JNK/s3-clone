# S3 Create Bucket (`PUT /{bucket}`)

## Overview
Creates a new S3 bucket. Requires authentication. Anonymous requests are not allowed. The bucket name must be globally unique.

---

## Request

### HTTP Method & Path
```
PUT /{bucket} HTTP/1.1
Host: localhost:9000
```

### Required Headers
- `Authorization`: AWS Signature V4
- `Date`: RFC 1123 date
- `Host`: `localhost:9000`

### Optional Headers
- `x-amz-acl`: `private | public-read | public-read-write | authenticated-read`
- `x-amz-bucket-object-lock-enabled`: `true | false`
- `x-amz-object-ownership`: `BucketOwnerPreferred | ObjectWriter | BucketOwnerEnforced`
- `x-amz-grant-full-control`, `x-amz-grant-read`, `x-amz-grant-read-acp`, `x-amz-grant-write`, `x-amz-grant-write-acp`: (ACL grants)

### Request Body (XML, optional)
If specifying a region:
```xml
<CreateBucketConfiguration xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <LocationConstraint>eu-west-1</LocationConstraint>
</CreateBucketConfiguration>
```
If omitted, defaults to `us-east-1`.

---

## Response

### Success
- **Status:** `200 OK`
- **Headers:**
  - `Location: /{bucket}`
- **Body:** Empty

#### Example
```
HTTP/1.1 200 OK
Location: /amzn-s3-demo-bucket
Content-Length: 0
```

### Error Cases
- **409 BucketAlreadyExists**: The requested bucket name is not available (global namespace).
- **409 BucketAlreadyOwnedByYou**: The bucket exists and is owned by you (except us-east-1, which returns 200 OK).
- **400 InvalidBucketName**: Bucket name does not conform to S3 naming rules.
- **403 AccessDenied**: Not authorized.

---

## Validation
- Bucket name must be globally unique and follow S3 naming rules:
  - 3-63 characters
  - Lowercase letters, numbers, and hyphens
  - Cannot be formatted as IP address
  - Cannot start or end with a hyphen
- If `LocationConstraint` is provided, must be a valid AWS region
- Only authenticated requests allowed
- If ACLs or object lock are specified, must have appropriate permissions

---

## References
- [AWS S3 CreateBucket API Docs](https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html)
- [S3 Bucket Naming Rules](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html)
