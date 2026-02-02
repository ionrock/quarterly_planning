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

(none yet)

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
