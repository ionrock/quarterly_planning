---
id: "test-006"
title: "Image Processing Pipeline"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build an image processing pipeline service. Supports resizing, format conversion, and optimization. Processes images asynchronously via job queue.

## Constraints

- Process 1000 images per minute
- Output quality visually indistinguishable from original

## Implementation Notes

- Python with Pillow and libvips
- Redis for job queue
- S3 for image storage

## Review Notes

(none yet)

## Tickets

### Ticket 1: Image Operations

**Summary:** Implement resize, crop, and format conversion.

**Definition of Done:** All operations produce correct output.

### Ticket 2: Job Queue

**Summary:** Async processing with Redis queue.

**Definition of Done:** Jobs processed reliably with retries.

### Ticket 3: Storage Integration

**Summary:** Read from and write to S3.

**Definition of Done:** Images stored with correct paths and metadata.
