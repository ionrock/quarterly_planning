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

(none yet)

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
