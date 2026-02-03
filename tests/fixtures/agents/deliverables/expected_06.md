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

#### Acceptance Criteria

1. **Resize Operation**
   - [ ] Resize to exact dimensions (width x height)
   - [ ] Resize by percentage (50%, 200%, etc.)
   - [ ] Resize with aspect ratio preservation (fit within bounds)
   - [ ] Upscaling uses Lanczos resampling
   - [ ] Downscaling uses high-quality antialiasing

2. **Crop Operation**
   - [ ] Crop to specific region (x, y, width, height)
   - [ ] Smart crop (face detection / content-aware)
   - [ ] Crop to aspect ratio from center
   - [ ] Handles images smaller than crop region

3. **Format Conversion**
   - [ ] Support JPEG, PNG, WebP, AVIF output
   - [ ] JPEG quality configurable (1-100, default 85)
   - [ ] PNG compression level configurable
   - [ ] WebP quality configurable, supports lossless
   - [ ] AVIF quality configurable

4. **Optimization**
   - [ ] Strip EXIF metadata (configurable)
   - [ ] Optimize output file size
   - [ ] Progressive JPEG for images > 10KB
   - [ ] Output within 5% of optimal file size

5. **Error Handling**
   - [ ] Invalid image file returns clear error
   - [ ] Unsupported format returns list of supported formats
   - [ ] Corrupted image handled gracefully

#### Demo Script
```python
from image_processor import ImageProcessor

processor = ImageProcessor()

# Resize
result = processor.resize("input.jpg", width=800, height=600)
assert result.size == (800, 600)

# Resize preserving aspect ratio
result = processor.resize("input.jpg", width=800, preserve_aspect=True)
assert result.size[0] == 800  # width is 800
# height calculated to preserve ratio

# Crop
result = processor.crop("input.jpg", x=100, y=100, width=500, height=500)
assert result.size == (500, 500)

# Convert to WebP
result = processor.convert("input.jpg", format="webp", quality=90)
assert result.format == "WEBP"

# Chain operations
result = processor.process("input.jpg", [
    {"op": "resize", "width": 1200},
    {"op": "crop", "aspect": "16:9"},
    {"op": "convert", "format": "webp", "quality": 85}
])
```

#### Test Requirements
- [ ] Unit tests for each operation
- [ ] Test with various input formats (JPEG, PNG, GIF, TIFF)
- [ ] Test edge cases (1x1 image, very large image)
- [ ] Visual regression tests comparing output quality
- [ ] Benchmark: resize 1000 images in under 60 seconds

### Ticket 2: Job Queue

**Summary:** Async processing with Redis queue.

**Definition of Done:** Jobs processed reliably with retries.

#### Acceptance Criteria

1. **Job Submission**
   - [ ] Jobs submitted via HTTP API
   - [ ] Job ID returned immediately (UUID)
   - [ ] Job payload validated before queueing
   - [ ] Duplicate detection (same input + operations)

2. **Queue Management**
   - [ ] Redis list used for job queue
   - [ ] Priority queues (high, normal, low)
   - [ ] Job TTL configurable (default 24 hours)
   - [ ] Dead letter queue for failed jobs

3. **Worker Processing**
   - [ ] Workers poll queue with BRPOPLPUSH
   - [ ] Processing moves job to "in-progress" list
   - [ ] Successful completion removes from in-progress
   - [ ] Worker count configurable (default: CPU count)

4. **Retry Logic**
   - [ ] Failed jobs retried 3 times
   - [ ] Exponential backoff (1s, 5s, 25s)
   - [ ] Permanent failures moved to dead letter queue
   - [ ] Retry count tracked in job metadata

5. **Job Status**
   - [ ] Status queryable by job ID
   - [ ] States: queued, processing, completed, failed
   - [ ] Progress percentage for multi-step jobs
   - [ ] Error message stored on failure

#### Demo Script
```bash
# Submit job
curl -X POST http://localhost:8000/jobs \
  -H "Content-Type: application/json" \
  -d '{
    "input": "s3://bucket/input.jpg",
    "operations": [{"op": "resize", "width": 800}],
    "output": "s3://bucket/output.jpg"
  }'
# Response: {"job_id": "abc-123", "status": "queued"}

# Check status
curl http://localhost:8000/jobs/abc-123
# Response: {"job_id": "abc-123", "status": "processing", "progress": 50}

# Wait for completion
curl http://localhost:8000/jobs/abc-123
# Response: {"job_id": "abc-123", "status": "completed", "output": "s3://bucket/output.jpg"}

# List failed jobs
curl http://localhost:8000/jobs/failed
# Response: [{"job_id": "def-456", "error": "Invalid image format", "retries": 3}]

# Retry failed job
curl -X POST http://localhost:8000/jobs/def-456/retry
```

#### Test Requirements
- [ ] Integration test with Redis
- [ ] Test job submission and completion
- [ ] Test retry on failure
- [ ] Test dead letter queue
- [ ] Load test: 1000 jobs queued simultaneously
- [ ] Test worker crash recovery

### Ticket 3: Storage Integration

**Summary:** Read from and write to S3.

**Definition of Done:** Images stored with correct paths and metadata.

#### Acceptance Criteria

1. **S3 Read**
   - [ ] Download image from S3 URI
   - [ ] Support s3:// URI scheme
   - [ ] Stream large images (don't load entirely in memory)
   - [ ] Handle missing file with clear error

2. **S3 Write**
   - [ ] Upload processed image to S3
   - [ ] Set Content-Type header correctly
   - [ ] Set Cache-Control header (configurable)
   - [ ] Support custom metadata

3. **Path Generation**
   - [ ] Output path templating: {hash}, {date}, {original_name}
   - [ ] Example: processed/{date}/{hash}.webp
   - [ ] Collision-free naming

4. **Presigned URLs**
   - [ ] Generate presigned GET URLs for output
   - [ ] URL expiration configurable (default 1 hour)
   - [ ] Generate presigned PUT URLs for direct upload

5. **Multi-Region Support**
   - [ ] Read from any S3 region
   - [ ] Write to configured region
   - [ ] Cross-region copy supported

#### Demo Script
```python
from storage import S3Storage

storage = S3Storage(
    bucket="my-images",
    region="us-east-1",
    path_template="processed/{date}/{hash}.{ext}"
)

# Download
image_data = storage.download("s3://my-images/uploads/photo.jpg")

# Upload
output_path = storage.upload(
    processed_image,
    original_name="photo.jpg",
    format="webp",
    metadata={"original_size": "1.2MB", "processed_at": "2024-01-15T10:00:00Z"}
)
# Returns: s3://my-images/processed/2024-01-15/abc123.webp

# Generate presigned URL
url = storage.presigned_url(output_path, expires_in=3600)
# Returns: https://my-images.s3.amazonaws.com/processed/...?X-Amz-Signature=...
```

#### Test Requirements
- [ ] Integration test with LocalStack or MinIO
- [ ] Test upload and download
- [ ] Test path templating
- [ ] Test presigned URL generation
- [ ] Test large file streaming (100MB+)
- [ ] Test error handling for missing files
