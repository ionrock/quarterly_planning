---
id: "test-017"
title: "Video Transcoding Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a video transcoding service that converts videos to multiple formats and resolutions. Supports HLS/DASH adaptive streaming output.

## Constraints

- Process 4K video in real-time
- Support all common input formats

## Implementation Notes

- FFmpeg for transcoding
- Kubernetes for scaling
- S3 for storage

## Review Notes

(none yet)

## Tickets

### Ticket 1: Transcoding Engine

**Summary:** FFmpeg wrapper with preset management.

**Definition of Done:** Videos transcoded with correct settings.

#### Acceptance Criteria

1. **Input Format Support**
   - [ ] MP4, MOV, AVI, MKV, WebM container formats
   - [ ] H.264, H.265, VP8, VP9, AV1 codecs
   - [ ] AAC, MP3, Opus audio codecs
   - [ ] Resolution up to 4K (3840x2160)
   - [ ] Frame rates up to 60fps

2. **Transcoding Presets**
   - [ ] Preset: name, codec, resolution, bitrate, audio settings
   - [ ] Built-in presets: 1080p, 720p, 480p, 360p
   - [ ] Custom presets via configuration
   - [ ] Two-pass encoding for quality

3. **FFmpeg Wrapper**
   - [ ] Build FFmpeg command from preset
   - [ ] Progress parsing from stderr
   - [ ] Error detection and classification
   - [ ] Hardware acceleration (NVENC, VAAPI) optional

4. **Output Quality**
   - [ ] CRF mode for quality-based encoding
   - [ ] ABR mode for streaming
   - [ ] Audio normalization
   - [ ] Deinterlacing if needed

5. **Performance**
   - [ ] 4K real-time with hardware acceleration
   - [ ] 1080p real-time on CPU
   - [ ] Memory-efficient streaming (no full file in memory)

#### Demo Script
```python
from transcoder import TranscodeEngine, Preset

engine = TranscodeEngine()

# List available presets
presets = engine.list_presets()
# ['hls_1080p', 'hls_720p', 'hls_480p', 'mp4_1080p', ...]

# Transcode with preset
result = await engine.transcode(
    input_path='s3://bucket/input.mov',
    output_path='s3://bucket/output.mp4',
    preset='mp4_1080p',
    progress_callback=lambda p: print(f'Progress: {p}%')
)
# Progress: 10%
# Progress: 50%
# Progress: 100%
# TranscodeResult(duration=45.2, output_size=150_000_000)

# Custom preset
custom = Preset(
    name='4k_hevc',
    video_codec='libx265',
    resolution='3840x2160',
    bitrate='15M',
    audio_codec='aac',
    audio_bitrate='256k'
)
await engine.transcode(input_path, output_path, preset=custom)

# Get video info
info = await engine.probe('s3://bucket/input.mov')
# VideoInfo(duration=120.5, resolution='1920x1080', codec='h264', fps=24)
```

#### Test Requirements
- [ ] Test each input format
- [ ] Test preset encoding settings
- [ ] Test progress reporting accuracy
- [ ] Test error handling (corrupt input)
- [ ] Benchmark: 1080p encoding speed
- [ ] Visual quality comparison (VMAF score > 90)

### Ticket 2: Adaptive Streaming

**Summary:** Generate HLS and DASH manifests.

**Definition of Done:** Streams play in video.js and hls.js.

#### Acceptance Criteria

1. **HLS Output**
   - [ ] Master playlist with all variants
   - [ ] Media playlists per quality level
   - [ ] Segment duration configurable (default 6s)
   - [ ] fMP4 segments (not TS) for better compatibility

2. **DASH Output**
   - [ ] MPD manifest
   - [ ] Multiple adaptation sets
   - [ ] Segment timeline
   - [ ] Compatible with dash.js

3. **Quality Ladder**
   - [ ] Default: 1080p, 720p, 480p, 360p
   - [ ] Bitrate: 5M, 2.5M, 1M, 500k
   - [ ] Configurable per job
   - [ ] Auto-detect source resolution (no upscaling)

4. **Segment Generation**
   - [ ] Key frame alignment across qualities
   - [ ] Consistent segment duration
   - [ ] Byte-range addressing support
   - [ ] Efficient segment naming

5. **DRM Support**
   - [ ] AES-128 encryption for HLS
   - [ ] Widevine/PlayReady for DASH (optional)
   - [ ] Key rotation support

#### Demo Script
```bash
# Generate HLS
curl -X POST http://localhost:8000/api/transcode \
  -d '{
    "input": "s3://bucket/video.mp4",
    "output": "s3://bucket/hls/video/",
    "format": "hls",
    "qualities": ["1080p", "720p", "480p"],
    "segment_duration": 6
  }'
# Response: {"job_id": "job-123", "status": "queued"}

# Output structure:
# s3://bucket/hls/video/
#   master.m3u8
#   1080p/
#     playlist.m3u8
#     segment_0000.m4s
#     segment_0001.m4s
#     ...
#   720p/
#     playlist.m3u8
#     ...

# Test playback in hls.js
# <video id="video"></video>
# <script>
#   const hls = new Hls();
#   hls.loadSource('https://cdn.example.com/video/master.m3u8');
#   hls.attachMedia(document.getElementById('video'));
# </script>

# Verify stream
ffprobe -i https://cdn.example.com/video/master.m3u8
# Shows all quality variants
```

#### Test Requirements
- [ ] Test HLS manifest structure
- [ ] Test DASH MPD structure
- [ ] Test segment alignment
- [ ] Test playback in hls.js
- [ ] Test playback in dash.js
- [ ] Test adaptive switching
- [ ] Test AES-128 encryption

### Ticket 3: Job Management

**Summary:** Queue and monitor transcoding jobs.

**Definition of Done:** Jobs tracked from submission to completion.

#### Acceptance Criteria

1. **Job Submission**
   - [ ] POST /api/jobs submits transcoding job
   - [ ] Returns job ID immediately
   - [ ] Input validation (format, settings)
   - [ ] Webhook URL for completion notification

2. **Job States**
   - [ ] queued: waiting for worker
   - [ ] downloading: fetching input from S3
   - [ ] transcoding: FFmpeg running
   - [ ] uploading: writing output to S3
   - [ ] completed: success
   - [ ] failed: error occurred

3. **Progress Tracking**
   - [ ] Overall progress percentage
   - [ ] Current stage
   - [ ] Estimated time remaining
   - [ ] Real-time updates via SSE

4. **Job Prioritization**
   - [ ] Priority levels: high, normal, low
   - [ ] High priority jobs processed first
   - [ ] Fair scheduling within priority

5. **Failure Handling**
   - [ ] Automatic retry on transient failures
   - [ ] Detailed error message on failure
   - [ ] Partial output cleanup on failure
   - [ ] Manual retry via API

#### Demo Script
```bash
# Submit job
curl -X POST http://localhost:8000/api/jobs \
  -H "Content-Type: application/json" \
  -d '{
    "input": "s3://bucket/raw/video.mov",
    "output": "s3://bucket/hls/video/",
    "preset": "hls_adaptive",
    "priority": "high",
    "webhook": "https://myapp.com/webhooks/transcode"
  }'
# Response: {"job_id": "job-abc123", "status": "queued"}

# Check status
curl http://localhost:8000/api/jobs/job-abc123
# {
#   "job_id": "job-abc123",
#   "status": "transcoding",
#   "progress": 45,
#   "stage": "encoding 720p variant",
#   "eta_seconds": 120,
#   "started_at": "2024-01-15T10:00:00Z"
# }

# Stream progress updates
curl -N http://localhost:8000/api/jobs/job-abc123/stream
# data: {"progress": 45, "stage": "encoding 720p variant"}
# data: {"progress": 50, "stage": "encoding 720p variant"}
# data: {"progress": 100, "stage": "completed"}

# List jobs
curl "http://localhost:8000/api/jobs?status=transcoding&limit=10"
# {"jobs": [...], "total": 25}

# Webhook payload on completion
# POST https://myapp.com/webhooks/transcode
# {
#   "job_id": "job-abc123",
#   "status": "completed",
#   "output_url": "s3://bucket/hls/video/master.m3u8",
#   "duration_seconds": 180,
#   "output_size_bytes": 500000000
# }
```

#### Test Requirements
- [ ] Test job submission and queuing
- [ ] Test status updates accuracy
- [ ] Test SSE progress streaming
- [ ] Test webhook delivery
- [ ] Test priority scheduling
- [ ] Test failure and retry
- [ ] Load test: 100 concurrent jobs
