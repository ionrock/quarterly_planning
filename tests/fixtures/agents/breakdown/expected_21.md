---
id: "test-021"
title: "Webhook Delivery System"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a webhook delivery system that reliably delivers HTTP callbacks to external endpoints. Supports retry with exponential backoff, payload signing, and delivery status tracking.

## Constraints

- Guarantee at-least-once delivery
- Support 1,000 webhooks per minute

## Implementation Notes

- Use Python with Celery
- Redis for queue
- PostgreSQL for tracking

## Review Notes

(none yet)

## Tickets

### Ticket 1: Webhook Queueing

**Summary:** Queue webhooks for delivery.

**Definition of Done:** Webhooks queued and processed.

#### Steps

1. **Create Python project**
   - Create webhooks/ package directory
   - Create pyproject.toml with dependencies
   - Verify: package installs

2. **Install Celery and Redis**
   - Add celery[redis] to dependencies
   - Verify: packages install

3. **Configure Celery app**
   - Create webhooks/celery.py
   - Configure broker URL from environment
   - Verify: Celery initializes

4. **Define WebhookPayload model**
   - Create webhooks/models.py
   - Include: url, headers, body, secret
   - Verify: model works

5. **Create deliver_webhook task**
   - Create webhooks/tasks.py
   - Define Celery task with bind=True
   - Verify: task registered

6. **Implement webhook enqueue function**
   - Create enqueue(payload) -> task_id
   - Call deliver_webhook.delay()
   - Verify: task queued

7. **Create API endpoint for enqueueing**
   - POST /webhooks/send accepts payload
   - Return task_id for tracking
   - Verify: endpoint works

8. **Add priority queue support**
   - Define high/normal/low priority queues
   - Route tasks based on priority parameter
   - Verify: priorities respected

9. **Configure worker concurrency**
   - Set worker concurrency for throughput
   - Verify: 1000/minute achievable

### Ticket 2: Delivery Logic

**Summary:** Deliver webhooks with retry.

**Definition of Done:** Webhooks delivered with backoff.

#### Steps

1. **Install httpx for HTTP client**
   - Add httpx to dependencies
   - Verify: package installs

2. **Implement basic HTTP delivery**
   - POST to webhook URL with body
   - Set Content-Type header
   - Return response status
   - Verify: webhooks delivered

3. **Implement HMAC signature**
   - Calculate HMAC-SHA256 of body with secret
   - Add X-Webhook-Signature header
   - Include timestamp in signature
   - Verify: signature correct

4. **Implement timeout handling**
   - Set connect and read timeouts (30s)
   - Catch timeout exceptions
   - Verify: timeouts handled

5. **Implement retry logic**
   - Configure max_retries=5 on task
   - Use exponential backoff: 1m, 5m, 30m, 2h, 24h
   - Verify: retries work

6. **Handle HTTP error responses**
   - Retry on 5xx errors
   - Retry on 429 (rate limited)
   - Don't retry on 4xx (except 429)
   - Verify: retry logic correct

7. **Handle network errors**
   - Catch connection errors
   - Retry with backoff
   - Verify: network errors retried

8. **Implement circuit breaker**
   - Track failures per endpoint
   - Stop sending after threshold
   - Verify: broken endpoints skipped

9. **Add request ID header**
   - Generate unique ID per delivery
   - Add X-Webhook-ID header
   - Verify: requests traceable

10. **Implement idempotency**
    - Include idempotency key in headers
    - Endpoint can deduplicate
    - Verify: key included

### Ticket 3: Status Tracking

**Summary:** Track delivery attempts.

**Definition of Done:** Delivery history queryable.

#### Steps

1. **Create database tables**
   - Create webhooks table: id, url, status, attempts, created_at
   - Create attempts table: id, webhook_id, status_code, error, created_at
   - Verify: migrations run

2. **Install SQLAlchemy**
   - Add sqlalchemy[asyncio] to dependencies
   - Verify: package installs

3. **Create database models**
   - Create Webhook ORM model
   - Create Attempt ORM model
   - Verify: models work

4. **Record webhook on enqueue**
   - Insert webhook record with status=pending
   - Return webhook ID
   - Verify: record created

5. **Record attempt before delivery**
   - Create attempt record with started_at
   - Verify: attempt recorded

6. **Update attempt after delivery**
   - Record status_code, response_body (truncated), duration
   - Verify: result recorded

7. **Update webhook status**
   - Set status=delivered on success
   - Set status=failed after max retries
   - Verify: status updated

8. **Create status query endpoint**
   - GET /webhooks/{id} returns status
   - Include attempt history
   - Verify: status queryable

9. **Create list endpoints**
   - GET /webhooks?status=failed lists webhooks
   - Support pagination
   - Verify: listing works

10. **Add metrics collection**
    - Track delivery success rate
    - Track average delivery time
    - Export to Prometheus
    - Verify: metrics exported

11. **Implement cleanup job**
    - Delete old webhook records (30 days)
    - Run as scheduled task
    - Verify: old records cleaned

12. **Add alerting**
    - Alert on high failure rate
    - Alert on queue backlog
    - Verify: alerts trigger
