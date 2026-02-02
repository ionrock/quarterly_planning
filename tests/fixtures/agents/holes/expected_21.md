---
id: "test-021"
title: "Webhook Delivery System"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a webhook delivery system that reliably delivers HTTP callbacks to external endpoints. Supports retry with exponential backoff, payload signing, and delivery status tracking. Built with Python and Celery.

## Constraints

- Guarantee at-least-once delivery
- Support 1,000 webhooks per minute

## Implementation Notes

- Queue webhooks in Redis via Celery
- Exponential backoff: 1min, 5min, 30min, 2hr, 24hr
- Sign payloads with HMAC-SHA256
- Store delivery attempts in PostgreSQL

## Review Notes

### Identified Weaknesses

1. **No endpoint validation**: Malicious users could use webhooks for SSRF attacks.

2. **Timeout handling undefined**: What's the HTTP timeout per delivery attempt?

3. **No circuit breaker**: Repeatedly failing endpoints should be temporarily disabled.

4. **Payload size limits missing**: Large payloads could cause memory/network issues.

5. **No IP allowlist/denylist**: Security control for outbound connections.

### Edge Cases

- Endpoint returns 3xx redirect—follow or fail?
- What HTTP methods are supported (POST only, or configurable)?
- How are self-signed SSL certificates handled?
- What if endpoint is extremely slow but eventually succeeds?
- Duplicate deliveries during retry—idempotency keys?
- What about IPv6-only endpoints?

### Assumptions to Validate

- Is there a maximum retry duration (give up after 24hr)?
- Should successful delivery result be stored, or only failures?
- Do endpoints need pre-registration/verification?
- Is manual retry/replay of webhooks needed?
- What's the payload format (JSON only, or flexible)?

### Potential Failures

- Redis/Celery broker unavailability
- Webhook floods overwhelming workers
- DNS resolution failures for endpoints
- SSL/TLS handshake failures
- Response body too large (memory exhaustion)
- Database write failure losing delivery records

## Tickets

### Ticket 1: Webhook Queueing

**Summary:** Queue webhook deliveries via Celery.

**Definition of Done:** Webhooks are queued and processed asynchronously.

### Ticket 2: Delivery Logic

**Summary:** Implement HTTP delivery with retry logic.

**Definition of Done:** Webhooks retry on failure with backoff.

### Ticket 3: Status Tracking

**Summary:** Record delivery attempts and status.

**Definition of Done:** Delivery history is queryable.
