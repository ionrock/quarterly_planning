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

### Technology Stack
- **Framework:** FastAPI for upload API
- **Image Processing:** Pillow 10.x
- **Task Queue:** Celery 5.x with Redis broker
- **Storage:** boto3 for S3 integration
- **Validation:** python-magic for MIME type detection

### Image Size Specifications
```python
IMAGE_SIZES = {
    'thumbnail': {'width': 150, 'height': 150, 'method': 'cover'},
    'medium': {'width': 800, 'height': 600, 'method': 'fit'},
    'large': {'width': 1920, 'height': 1080, 'method': 'fit'},
}

QUALITY_SETTINGS = {
    'jpeg': {'quality': 85, 'optimize': True},
    'png': {'optimize': True, 'compress_level': 6},
    'webp': {'quality': 85, 'method': 4},
}
```

### Upload API Endpoint
```python
@router.post("/images/upload")
async def upload_image(
    file: UploadFile = File(...),
    background_tasks: BackgroundTasks,
) -> ImageUploadResponse:
    # Validate file type
    content = await file.read(8192)
    mime_type = magic.from_buffer(content, mime=True)
    if mime_type not in ['image/jpeg', 'image/png', 'image/webp']:
        raise HTTPException(400, f"Unsupported image type: {mime_type}")

    # Read full file
    await file.seek(0)
    content = await file.read()

    if len(content) > 10 * 1024 * 1024:  # 10MB limit
        raise HTTPException(400, "File too large (max 10MB)")

    # Generate job ID and store temporarily
    job_id = str(uuid.uuid4())
    temp_path = Path(f"/tmp/uploads/{job_id}")
    temp_path.write_bytes(content)

    # Queue processing task
    process_image.delay(job_id, file.filename, mime_type)

    return ImageUploadResponse(
        job_id=job_id,
        status="processing",
        status_url=f"/images/status/{job_id}",
    )
```

### Celery Task Definition
```python
@celery.task(bind=True, max_retries=3, default_retry_delay=60)
def process_image(self, job_id: str, filename: str, mime_type: str):
    try:
        temp_path = Path(f"/tmp/uploads/{job_id}")
        results = {}

        with Image.open(temp_path) as img:
            # Fix EXIF orientation
            img = ImageOps.exif_transpose(img)

            # Convert to RGB if necessary (for JPEG output)
            if img.mode in ('RGBA', 'P'):
                img = img.convert('RGB')

            for size_name, spec in IMAGE_SIZES.items():
                processed = resize_image(img, spec)
                output_format = 'webp'  # Always output WebP for web

                # Save to buffer
                buffer = BytesIO()
                processed.save(buffer, format='WEBP', **QUALITY_SETTINGS['webp'])
                buffer.seek(0)

                # Upload to S3
                s3_key = f"images/{job_id}/{size_name}.webp"
                s3_client.upload_fileobj(
                    buffer,
                    settings.S3_BUCKET,
                    s3_key,
                    ExtraArgs={
                        'ContentType': 'image/webp',
                        'CacheControl': 'public, max-age=31536000',
                    }
                )

                results[size_name] = {
                    'url': f"{settings.CDN_URL}/{s3_key}",
                    'width': processed.width,
                    'height': processed.height,
                }

        # Update job status in Redis
        redis_client.hset(f"image_job:{job_id}", mapping={
            'status': 'completed',
            'results': json.dumps(results),
            'completed_at': datetime.utcnow().isoformat(),
        })

        # Cleanup temp file
        temp_path.unlink(missing_ok=True)

    except Exception as e:
        redis_client.hset(f"image_job:{job_id}", mapping={
            'status': 'failed',
            'error': str(e),
        })
        raise self.retry(exc=e)
```

### Resize Algorithm
```python
def resize_image(img: Image.Image, spec: dict) -> Image.Image:
    target_w, target_h = spec['width'], spec['height']
    method = spec['method']

    if method == 'cover':
        # Crop to fill exact dimensions
        img_ratio = img.width / img.height
        target_ratio = target_w / target_h

        if img_ratio > target_ratio:
            # Image is wider - crop width
            new_width = int(img.height * target_ratio)
            left = (img.width - new_width) // 2
            img = img.crop((left, 0, left + new_width, img.height))
        else:
            # Image is taller - crop height
            new_height = int(img.width / target_ratio)
            top = (img.height - new_height) // 2
            img = img.crop((0, top, img.width, top + new_height))

        return img.resize((target_w, target_h), Image.Resampling.LANCZOS)

    elif method == 'fit':
        # Fit within dimensions, preserve aspect ratio
        img.thumbnail((target_w, target_h), Image.Resampling.LANCZOS)
        return img

    else:
        raise ValueError(f"Unknown resize method: {method}")
```

### Job Status Endpoint
```python
@router.get("/images/status/{job_id}")
async def get_status(job_id: str) -> ImageStatusResponse:
    job_data = redis_client.hgetall(f"image_job:{job_id}")
    if not job_data:
        raise HTTPException(404, "Job not found")

    return ImageStatusResponse(
        job_id=job_id,
        status=job_data.get('status', 'processing'),
        results=json.loads(job_data.get('results', '{}')),
        error=job_data.get('error'),
    )
```

### S3 Configuration
```python
s3_client = boto3.client(
    's3',
    region_name=settings.AWS_REGION,
    aws_access_key_id=settings.AWS_ACCESS_KEY_ID,
    aws_secret_access_key=settings.AWS_SECRET_ACCESS_KEY,
)

# Bucket policy should allow public read for processed images
# CloudFront distribution recommended for CDN delivery
```

## Review Notes

(none yet)

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
