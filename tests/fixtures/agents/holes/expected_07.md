---
id: "test-007"
title: "Image Processing Pipeline"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build an image processing pipeline that resizes, crops, and optimizes images for web delivery. Accepts uploads via API, processes them asynchronously, and stores results in S3. Built with Python and Celery.

## Constraints

- Process images within 30 seconds
- Support JPEG, PNG, and WebP formats

## Implementation Notes

- Use Pillow for image manipulation
- Celery with Redis for async task queue
- Generate multiple sizes (thumbnail, medium, large)
- Upload processed images to S3

## Review Notes

### Identified Weaknesses

1. **No file validation**: Malicious files disguised as images could be uploaded. Need magic byte validation.

2. **Missing size limits**: No maximum file size mentioned—could lead to resource exhaustion.

3. **No job status tracking**: How does the client know when processing is complete?

4. **Temporary storage cleanup**: Where are uploads stored before processing? When are they cleaned up?

5. **No retry strategy**: What happens if S3 upload fails or processing crashes?

### Edge Cases

- What about images with EXIF rotation metadata?
- How are animated GIFs/WebPs handled? (Multiple frames)
- What if the uploaded file isn't actually an image despite extension?
- Very small images that would upscale—skip or stretch?
- Images with transparency (PNG/WebP to JPEG conversion)?
- CMYK color space images?
- Extremely large dimension images (e.g., 20000x20000)?

### Assumptions to Validate

- What are the exact output dimensions for thumbnail/medium/large?
- Is the S3 bucket already configured, including permissions?
- Should original images be preserved or deleted after processing?
- What compression quality levels should be used?
- Is there a CDN in front of S3?

### Potential Failures

- Worker crashes mid-processing (corrupted output)
- S3 upload timeout on large files
- Redis/Celery broker unavailability
- Disk space exhaustion from temporary files
- Memory exhaustion processing very large images
- Task queue backlog during traffic spikes

## Tickets

### Ticket 1: Upload API

**Summary:** Create endpoint to accept image uploads.

**Definition of Done:** Images can be uploaded and stored temporarily.

### Ticket 2: Processing Workers

**Summary:** Implement Celery workers for image processing.

**Definition of Done:** Workers resize and optimize images correctly.

### Ticket 3: S3 Integration

**Summary:** Upload processed images to S3 and return URLs.

**Definition of Done:** Images are accessible via S3 URLs.
