# S3 Bucket ACLs & CORS

## Overview
Controls access to buckets and objects using ACLs and CORS rules. ACLs can be configured for public access, IP-based access, and CORS per bucket via the config file. CORS preflight requests are supported.

---

## ACLs
- Configured in the server config file under `bucket_acls`.
- Supported ACL types:
  - `public`: Bucket is publicly readable (optionally writable)
  - `allowed_ips`: Only requests from specified IPs/subnets are allowed
- ACLs are enforced for all relevant operations (GET, PUT, DELETE, etc.)

### Example Config
```yaml
bucket_acls:
  - bucket: "public-bucket"
    public: true
    cors:
      allowed_origins: ["*"]
      allowed_methods: ["GET", "PUT"]
  - bucket: "private-bucket"
    allowed_ips: ["192.168.1.0/24"]
```

---

## CORS
- CORS rules are configured per bucket in the config file.
- CORS preflight requests use the `OPTIONS` method.
- CORS headers are added to responses as configured.

### CORS Preflight Request
```
OPTIONS /{bucket}/{object} HTTP/1.1
Host: localhost:9000
Origin: https://example.com
Access-Control-Request-Method: GET
Access-Control-Request-Headers: Authorization
```

### CORS Preflight Response (Success)
```
HTTP/1.1 200 OK
Access-Control-Allow-Origin: https://example.com
Access-Control-Allow-Methods: GET, PUT
Access-Control-Allow-Headers: Authorization
Access-Control-Max-Age: 3600
Content-Length: 0
```

### Error Cases
- **403 AccessDenied**: Not authorized by ACL or CORS rules.

---

## Validation
- ACLs and CORS rules are validated on config reload
- Requests are checked against ACLs and CORS rules for each operation

---

## Example Error Responses
See `docs/examples/acl_cors/` for XML examples (to be created).

---

## Notes
- Uses path-style URLs on `localhost`
- Default region is `de-muc-01` (configurable via config)
- Optional headers are ignored unless a related feature is implemented

---

## References
- [AWS S3 CORS Docs](https://docs.aws.amazon.com/AmazonS3/latest/userguide/cors.html)
- [AWS S3 ACL Docs](https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html)
