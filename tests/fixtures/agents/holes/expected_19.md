---
id: "test-019"
title: "S3-Compatible Object Storage"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build an S3-compatible object storage server for development and testing. Supports basic S3 operations (GET, PUT, DELETE, LIST) with bucket management. Stores objects on local filesystem. Written in Go.

## Constraints

- Compatible with AWS SDK
- Support objects up to 5GB

## Implementation Notes

- Implement S3 REST API subset
- Store objects as files, metadata in SQLite
- Support multipart uploads for large files
- Implement bucket ACLs for access control

## Review Notes

### Identified Weaknesses

1. **AWS Signature V4 authentication not mentioned**: SDKs require proper auth; critical for compatibility.

2. **No presigned URL support**: Common pattern for direct uploads/downloads.

3. **Concurrent access handling unclear**: Multiple writers to same key? Read during write?

4. **Incomplete multipart cleanup**: Orphaned parts from abandoned uploads need cleanup.

5. **No versioning mentioned**: S3 versioning is common; at least need to handle unversioned gracefully.

### Edge Cases

- Object keys with special characters (spaces, unicode, slashes)
- Empty objects (zero bytes)
- Very long object keys (up to 1024 bytes in S3)
- Bucket names with periods (SSL certificate implications)
- Concurrent multipart uploads to same key
- List operations with many objects (pagination with continuation token)

### Assumptions to Validate

- Which S3 API version(s) need to be supported?
- Is path-style or virtual-hosted-style addressing needed?
- Do we need to support bucket policies (vs just ACLs)?
- Is server-side encryption in scope?
- What about object lifecycle policies?

### Potential Failures

- Filesystem full during upload
- SQLite lock contention under load
- Partial write on crash (data corruption)
- Disk I/O blocking request handlers
- File descriptor exhaustion with many concurrent requests
- Metadata/file mismatch after failed operations

## Tickets

### Ticket 1: Basic Operations

**Summary:** Implement GET, PUT, DELETE for objects.

**Definition of Done:** Basic object operations work with AWS CLI.

### Ticket 2: Bucket Management

**Summary:** Implement bucket create, delete, and list operations.

**Definition of Done:** Buckets can be managed via API.

### Ticket 3: Multipart Upload

**Summary:** Implement multipart upload for large files.

**Definition of Done:** Files over 5MB can be uploaded in parts.
