---
id: "test-019"
title: "S3-Compatible Storage Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build an S3-compatible object storage service with basic operations. Supports bucket management, object CRUD, and presigned URLs. Uses local filesystem as backend.

## Constraints

- Compatible with AWS S3 SDKs
- Handle files up to 5GB

## Implementation Notes

- Written in Go
- AWS Signature V4 authentication
- Filesystem-based storage

## Review Notes

(none yet)

## Tickets

### Ticket 1: Bucket Operations

**Summary:** Implement bucket create, list, and delete.

**Definition of Done:** Buckets manageable via S3 API.

#### Steps

1. **Create Go project**
   - Run `go mod init github.com/example/s3compat`
   - Create cmd/server/main.go
   - Verify: project builds

2. **Create storage directory structure**
   - Define DATA_DIR environment variable
   - Create base directory if not exists
   - Verify: directory created

3. **Define Bucket struct**
   - Include: Name, CreatedAt
   - Create internal/models/bucket.go
   - Verify: struct compiles

4. **Implement bucket storage**
   - Store bucket metadata in buckets.json
   - Create directory per bucket
   - Verify: storage works

5. **Create HTTP router**
   - Use chi or standard library
   - Route based on path and method
   - Verify: server starts

6. **Implement PUT /{bucket} (CreateBucket)**
   - Validate bucket name (DNS rules)
   - Create bucket directory
   - Update metadata file
   - Return 200 on success
   - Verify: bucket created

7. **Implement GET / (ListBuckets)**
   - Return XML with all buckets
   - Format as S3 ListAllMyBucketsResult
   - Verify: AWS SDK can list buckets

8. **Implement HEAD /{bucket} (HeadBucket)**
   - Return 200 if exists, 404 if not
   - Verify: bucket existence checkable

9. **Implement DELETE /{bucket} (DeleteBucket)**
   - Check bucket is empty
   - Return 409 BucketNotEmpty if not
   - Delete directory and metadata
   - Verify: empty bucket deleted

10. **Handle bucket name validation**
    - 3-63 characters
    - Lowercase letters, numbers, hyphens
    - Return 400 InvalidBucketName on invalid
    - Verify: validation works

### Ticket 2: Object Operations

**Summary:** Implement object PUT, GET, DELETE.

**Definition of Done:** Objects storable and retrievable.

#### Steps

1. **Define Object struct**
   - Include: Key, Size, ContentType, ETag, LastModified
   - Create internal/models/object.go
   - Verify: struct compiles

2. **Implement object key to path mapping**
   - Handle keys with slashes
   - Create subdirectories as needed
   - Verify: any key storable

3. **Store object metadata**
   - Create .meta file alongside object
   - Store Content-Type, custom headers
   - Verify: metadata persisted

4. **Implement PUT /{bucket}/{key} (PutObject)**
   - Read request body
   - Calculate MD5 for ETag
   - Write to file system
   - Store metadata
   - Return ETag header
   - Verify: object stored

5. **Handle large uploads**
   - Stream to disk without loading in memory
   - Use io.Copy with buffering
   - Verify: 5GB upload works

6. **Implement GET /{bucket}/{key} (GetObject)**
   - Check object exists
   - Return 404 NoSuchKey if not
   - Stream file content
   - Set Content-Type, Content-Length, ETag
   - Verify: object downloadable

7. **Implement HEAD /{bucket}/{key} (HeadObject)**
   - Return metadata without body
   - Set Content-Length, ETag, Last-Modified
   - Verify: metadata retrievable

8. **Implement DELETE /{bucket}/{key} (DeleteObject)**
   - Delete file and metadata
   - Return 204 on success
   - Return 204 even if not exists (S3 behavior)
   - Verify: object deleted

9. **Implement GET /{bucket}?list-type=2 (ListObjectsV2)**
   - List objects in bucket
   - Support prefix filtering
   - Support pagination with continuation token
   - Return XML response
   - Verify: AWS SDK can list objects

10. **Implement Range requests**
    - Parse Range header
    - Return partial content with 206
    - Verify: range requests work

11. **Handle conditional requests**
    - If-None-Match with ETag
    - If-Modified-Since
    - Return 304 Not Modified when appropriate
    - Verify: conditional requests work

### Ticket 3: Authentication

**Summary:** Implement AWS Signature V4.

**Definition of Done:** Requests authenticated like S3.

#### Steps

1. **Create credentials store**
   - Store access key ID and secret key pairs
   - Load from config file or environment
   - Verify: credentials loadable

2. **Parse Authorization header**
   - Extract algorithm, credential, signed headers, signature
   - Verify: header parsed correctly

3. **Extract canonical request components**
   - Method, URI, query string, headers, payload hash
   - Verify: components extracted

4. **Build canonical request string**
   - Format per AWS specification
   - Verify: matches AWS examples

5. **Build string to sign**
   - Include algorithm, timestamp, credential scope, canonical request hash
   - Verify: matches AWS examples

6. **Calculate signing key**
   - Derive from secret key, date, region, service
   - Use HMAC-SHA256 chain
   - Verify: key derivation correct

7. **Calculate signature**
   - HMAC-SHA256 of string to sign with signing key
   - Verify: signature matches AWS examples

8. **Create authentication middleware**
   - Extract and verify signature
   - Return 403 AccessDenied on failure
   - Verify: unsigned requests rejected

9. **Handle query string authentication**
   - Support X-Amz-Signature in query params
   - Used for presigned URLs
   - Verify: presigned URLs work

10. **Implement presigned URL generation**
    - Create endpoint to generate presigned URLs
    - Include expiration time
    - Verify: presigned URLs work with AWS SDK

11. **Add request timestamp validation**
    - Reject requests older than 15 minutes
    - Return 403 RequestTimeTooSkewed
    - Verify: old requests rejected

12. **Test with AWS SDK**
    - Configure AWS SDK to use local endpoint
    - Test all operations
    - Verify: SDK works without modification
