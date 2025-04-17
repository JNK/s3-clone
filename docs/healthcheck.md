# Healthcheck Endpoint

## Overview
The healthcheck endpoint provides a simple way to verify that the S3-compatible server is running and responsive. It is intended for use by load balancers, monitoring systems, or orchestration tools.

---

## Endpoint
- **Path:** `/healthz`
- **Method:** `GET`
- **Authentication:** Not required
- **Headers:** No special headers required

---

## Example Request
```
GET /healthz HTTP/1.1
Host: localhost:9000
```

---

## Example Response
```
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 2

OK
```

---

## Notes
- The endpoint returns `200 OK` and the body `OK` if the service is healthy.
- No authentication or S3-specific headers are required.
- Can be used by Kubernetes, Docker, or other orchestrators for liveness/readiness checks.
