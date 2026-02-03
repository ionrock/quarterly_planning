---
id: "test-015"
title: "Container Registry"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a container image registry compatible with Docker and OCI standards. Supports push, pull, and image lifecycle management. Includes vulnerability scanning.

## Constraints

- Support images up to 10GB
- Pull latency under 1 second for cached layers

## Implementation Notes

- Go implementation
- S3 for blob storage
- PostgreSQL for metadata

## Review Notes

(none yet)

## Tickets

### Ticket 1: Registry API

**Summary:** Implement OCI Distribution Spec endpoints.

**Definition of Done:** Docker push/pull works correctly.

#### Acceptance Criteria

1. **Base Endpoints**
   - [ ] GET /v2/ returns 200 (API version check)
   - [ ] Returns WWW-Authenticate header for auth
   - [ ] Supports both Docker and OCI media types

2. **Manifest Operations**
   - [ ] GET /v2/{name}/manifests/{reference} retrieves manifest
   - [ ] PUT /v2/{name}/manifests/{reference} uploads manifest
   - [ ] DELETE /v2/{name}/manifests/{reference} deletes manifest
   - [ ] HEAD /v2/{name}/manifests/{reference} checks existence
   - [ ] Reference can be tag or digest

3. **Blob Operations**
   - [ ] GET /v2/{name}/blobs/{digest} retrieves blob
   - [ ] HEAD /v2/{name}/blobs/{digest} checks existence
   - [ ] DELETE /v2/{name}/blobs/{digest} deletes blob
   - [ ] POST /v2/{name}/blobs/uploads/ initiates upload
   - [ ] PATCH /v2/{name}/blobs/uploads/{uuid} uploads chunk
   - [ ] PUT /v2/{name}/blobs/uploads/{uuid}?digest= completes upload

4. **Chunked Upload**
   - [ ] Support resumable chunked uploads
   - [ ] Track upload progress via Range header
   - [ ] Timeout incomplete uploads after 1 hour
   - [ ] Verify digest on completion

5. **Catalog**
   - [ ] GET /v2/_catalog lists repositories
   - [ ] GET /v2/{name}/tags/list lists tags
   - [ ] Pagination with n and last parameters

#### Demo Script
```bash
# Configure Docker to use registry
docker login localhost:5000 -u admin -p secret

# Push image
docker tag alpine:latest localhost:5000/myproject/alpine:v1
docker push localhost:5000/myproject/alpine:v1
# Pushing...
# v1: digest: sha256:abc123... size: 528

# Pull image
docker pull localhost:5000/myproject/alpine:v1
# Pulling...
# Status: Downloaded newer image

# List repositories
curl -u admin:secret http://localhost:5000/v2/_catalog
# {"repositories": ["myproject/alpine"]}

# List tags
curl -u admin:secret http://localhost:5000/v2/myproject/alpine/tags/list
# {"name": "myproject/alpine", "tags": ["v1", "latest"]}

# Get manifest
curl -u admin:secret \
  -H "Accept: application/vnd.oci.image.manifest.v1+json" \
  http://localhost:5000/v2/myproject/alpine/manifests/v1
# {manifest JSON}
```

#### Test Requirements
- [ ] Test push/pull with Docker CLI
- [ ] Test manifest operations
- [ ] Test chunked blob upload
- [ ] Test catalog pagination
- [ ] Test invalid digest rejection
- [ ] Conformance test with OCI distribution-spec

### Ticket 2: Storage Backend

**Summary:** Store blobs in S3 with caching.

**Definition of Done:** Blobs stored and retrieved efficiently.

#### Acceptance Criteria

1. **S3 Storage**
   - [ ] Blobs stored by content-addressable digest
   - [ ] Path: /blobs/{algorithm}/{digest}
   - [ ] Manifests stored separately: /manifests/{repo}/{digest}
   - [ ] Configurable bucket and prefix

2. **Upload Handling**
   - [ ] Chunked uploads assembled in temp location
   - [ ] Move to final path on completion
   - [ ] Clean up incomplete uploads
   - [ ] Deduplication via content-addressing

3. **Caching Layer**
   - [ ] Redis cache for manifest metadata
   - [ ] Local disk cache for hot layers (LRU)
   - [ ] Cache size configurable
   - [ ] Cache invalidation on delete

4. **Performance**
   - [ ] Stream blobs directly from S3 (no buffering)
   - [ ] Parallel chunk uploads
   - [ ] Connection pooling to S3
   - [ ] Presigned URLs for direct S3 download (optional)

5. **Garbage Collection**
   - [ ] Mark-and-sweep for unreferenced blobs
   - [ ] Dry-run mode shows what would be deleted
   - [ ] Schedule via cron or manual trigger
   - [ ] Protect recently uploaded blobs (grace period)

#### Demo Script
```bash
# Configure storage
cat > config.yaml << 'EOF'
storage:
  backend: s3
  s3:
    bucket: my-registry
    region: us-east-1
    prefix: registry/
  cache:
    type: redis
    redis_url: redis://localhost:6379
    disk_path: /var/cache/registry
    disk_size_gb: 50
EOF

# Start registry
registry serve --config config.yaml

# Push large image
docker push localhost:5000/myproject/large-image:v1
# Layers uploaded to S3, metadata cached in Redis

# Pull (cache hit)
docker pull localhost:5000/myproject/large-image:v1
# Cache HIT for layer sha256:abc... (served from disk)
# Cache MISS for layer sha256:def... (fetched from S3, cached)

# Garbage collection (dry run)
registry gc --dry-run
# Would delete 15 unreferenced blobs (2.3 GB)

# Garbage collection (actual)
registry gc --delete
# Deleted 15 blobs, reclaimed 2.3 GB
```

#### Test Requirements
- [ ] Test S3 upload and download
- [ ] Test chunked upload assembly
- [ ] Test cache hit/miss behavior
- [ ] Test garbage collection
- [ ] Load test: 100 concurrent pulls
- [ ] Benchmark: pull latency with/without cache

### Ticket 3: Image Scanning

**Summary:** Scan images for vulnerabilities.

**Definition of Done:** Vulnerabilities detected and reported.

#### Acceptance Criteria

1. **Scan Trigger**
   - [ ] Automatic scan on push
   - [ ] Manual scan via API
   - [ ] Rescan on demand (new vulnerability data)
   - [ ] Skip scan for already-scanned digest

2. **Vulnerability Detection**
   - [ ] Integrate with Trivy or Clair
   - [ ] Scan all layers
   - [ ] Detect OS package vulnerabilities
   - [ ] Detect application dependencies (npm, pip, etc.)

3. **Scan Results**
   - [ ] Severity levels: Critical, High, Medium, Low
   - [ ] CVE ID and description
   - [ ] Affected package and version
   - [ ] Fixed version (if available)
   - [ ] CVSS score

4. **API Endpoints**
   - [ ] GET /v2/{name}/manifests/{ref}/scan returns results
   - [ ] POST /v2/{name}/manifests/{ref}/scan triggers scan
   - [ ] GET /v2/{name}/scan/summary returns repo summary

5. **Policy Enforcement**
   - [ ] Block pull of images with critical vulnerabilities
   - [ ] Configurable severity threshold
   - [ ] Allowlist for known issues
   - [ ] Webhook on new vulnerabilities found

#### Demo Script
```bash
# Push image (triggers scan)
docker push localhost:5000/myproject/app:v1
# Pushed, scanning for vulnerabilities...

# Check scan status
curl -u admin:secret \
  http://localhost:5000/v2/myproject/app/manifests/v1/scan
# {
#   "status": "completed",
#   "scanned_at": "2024-01-15T10:00:00Z",
#   "vulnerabilities": {
#     "critical": 2,
#     "high": 5,
#     "medium": 12,
#     "low": 23
#   },
#   "details": [
#     {
#       "cve": "CVE-2024-1234",
#       "severity": "critical",
#       "package": "openssl",
#       "installed_version": "1.1.1k",
#       "fixed_version": "1.1.1l",
#       "description": "Buffer overflow in..."
#     }
#   ]
# }

# Get repository summary
curl -u admin:secret http://localhost:5000/v2/myproject/app/scan/summary
# {
#   "latest_tag": "v1",
#   "last_scanned": "2024-01-15T10:00:00Z",
#   "total_vulnerabilities": 42,
#   "critical_count": 2
# }

# Try to pull blocked image
docker pull localhost:5000/myproject/vulnerable:v1
# Error: image blocked due to critical vulnerabilities
```

#### Test Requirements
- [ ] Test scan trigger on push
- [ ] Test vulnerability detection accuracy
- [ ] Test scan result API
- [ ] Test policy enforcement (block pull)
- [ ] Test rescan with updated vulnerability data
- [ ] Integration test with Trivy
