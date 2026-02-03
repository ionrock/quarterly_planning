---
id: "test-007"
title: "Image Processing Pipeline"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create an image processing pipeline that handles uploads, generates thumbnails, and optimizes images for web delivery. Supports JPEG, PNG, and WebP formats.

## Constraints

- Process images under 5 seconds
- Maintain image quality above 85%

## Implementation Notes

- Use Sharp for image manipulation
- S3 for storage
- Queue-based processing

## Review Notes

(none yet)

## Tickets

### Ticket 1: Upload Handling

**Summary:** Accept and validate image uploads.

**Definition of Done:** Images upload to temporary storage with validation.

#### Steps

1. **Install upload dependencies**
   - Run `npm install multer`
   - Add @types/multer for TypeScript
   - Verify: packages installed

2. **Configure multer for memory storage**
   - Create src/upload/config.ts
   - Use memoryStorage() for buffer access
   - Set file size limit to 10MB
   - Verify: multer configured

3. **Create file filter for images**
   - Accept only image/jpeg, image/png, image/webp mimetypes
   - Reject other file types with 400 error
   - Verify: non-images rejected

4. **Create upload endpoint**
   - POST /images/upload accepts multipart form
   - Use multer.single('image') middleware
   - Verify: file available in req.file

5. **Validate image dimensions**
   - Use Sharp to get image metadata
   - Reject images smaller than 100x100 pixels
   - Reject images larger than 10000x10000 pixels
   - Verify: dimension validation works

6. **Generate unique file identifier**
   - Create UUID for each upload
   - Use as reference for processing pipeline
   - Verify: unique IDs generated

7. **Store original to temporary location**
   - Write buffer to /tmp/{uuid}/original.{ext}
   - Set cleanup job for orphaned files
   - Verify: file written to disk

8. **Create upload response**
   - Return { uploadId, originalSize, dimensions, format }
   - Include processing status URL
   - Verify: response contains expected fields

9. **Add upload progress tracking**
   - Create upload_jobs table with status
   - Status: pending, processing, complete, failed
   - Verify: job record created on upload

### Ticket 2: Image Processing

**Summary:** Resize, optimize, and convert images.

**Definition of Done:** Thumbnails generated, images optimized.

#### Steps

1. **Install Sharp**
   - Run `npm install sharp`
   - Verify: Sharp installed and working

2. **Create processing job queue**
   - Install and configure BullMQ
   - Create 'image-processing' queue
   - Verify: queue created in Redis

3. **Create thumbnail sizes configuration**
   - Define sizes: small (150x150), medium (400x400), large (800x800)
   - Store in src/processing/sizes.ts
   - Verify: sizes exported correctly

4. **Implement thumbnail generation**
   - Create src/processing/thumbnail.ts
   - Use Sharp resize with fit: 'cover'
   - Maintain aspect ratio, center crop
   - Verify: thumbnails generated at correct sizes

5. **Implement JPEG optimization**
   - Use Sharp jpeg({ quality: 85, mozjpeg: true })
   - Strip metadata (EXIF) unless requested
   - Verify: output smaller than input, quality acceptable

6. **Implement PNG optimization**
   - Use Sharp png({ compressionLevel: 9 })
   - Verify: output smaller than input

7. **Implement WebP conversion**
   - Generate WebP version for each size
   - Use Sharp webp({ quality: 85 })
   - Verify: WebP files generated

8. **Create processing worker**
   - Create src/workers/imageProcessor.ts
   - Process jobs from queue
   - Generate all thumbnails and optimized versions
   - Verify: worker processes queued jobs

9. **Update job status during processing**
   - Set status to 'processing' when starting
   - Update progress percentage
   - Set status to 'complete' or 'failed' when done
   - Verify: status updates visible

10. **Handle processing errors**
    - Catch Sharp errors (corrupt images, etc.)
    - Mark job as failed with error message
    - Clean up partial outputs
    - Verify: errors handled gracefully

### Ticket 3: Storage Integration

**Summary:** Store processed images in S3.

**Definition of Done:** Images accessible via CDN URLs.

#### Steps

1. **Install AWS SDK**
   - Run `npm install @aws-sdk/client-s3`
   - Verify: SDK installed

2. **Configure S3 client**
   - Create src/storage/s3.ts
   - Read AWS_REGION, AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY from env
   - Read S3_BUCKET from env
   - Verify: client initializes

3. **Create upload function**
   - Create uploadToS3(key, buffer, contentType): Promise<string>
   - Return S3 URL
   - Verify: file uploads successfully

4. **Define S3 key structure**
   - Format: images/{uploadId}/{size}.{format}
   - Example: images/abc123/small.webp
   - Verify: keys generated correctly

5. **Upload all processed versions**
   - After processing, upload original, thumbnails, and WebP versions
   - Run uploads in parallel (Promise.all)
   - Verify: all versions uploaded

6. **Set correct content types**
   - Set Content-Type header based on format
   - image/jpeg, image/png, image/webp
   - Verify: content types correct in S3

7. **Set cache control headers**
   - Set Cache-Control: public, max-age=31536000
   - Enable browser and CDN caching
   - Verify: headers set on S3 objects

8. **Configure CloudFront distribution**
   - Create CDN_URL environment variable
   - Generate public URLs using CDN domain
   - Verify: images accessible via CDN

9. **Save URLs to database**
   - Create images table: id, upload_id, urls (JSONB), created_at
   - Store all variant URLs
   - Verify: URLs queryable from database

10. **Create image retrieval endpoint**
    - GET /images/:uploadId returns all URLs
    - Include status and metadata
    - Verify: endpoint returns image data

11. **Implement cleanup of temp files**
    - Delete /tmp/{uuid}/ after S3 upload
    - Schedule cleanup job for failed uploads
    - Verify: temp files cleaned up
