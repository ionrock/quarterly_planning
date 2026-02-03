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

### Ticket 2: Object Operations

**Summary:** Implement object PUT, GET, DELETE.

**Definition of Done:** Objects storable and retrievable.

### Ticket 3: Authentication

**Summary:** Implement AWS Signature V4.

**Definition of Done:** Requests authenticated like S3.
