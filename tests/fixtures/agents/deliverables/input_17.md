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

### Ticket 2: Adaptive Streaming

**Summary:** Generate HLS and DASH manifests.

**Definition of Done:** Streams play in video.js and hls.js.

### Ticket 3: Job Management

**Summary:** Queue and monitor transcoding jobs.

**Definition of Done:** Jobs tracked from submission to completion.
