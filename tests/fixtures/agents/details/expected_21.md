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

### Technology Stack
- **Framework:** FastAPI for management API
- **Task Queue:** Celery 5.x with Redis broker
- **Database:** PostgreSQL for delivery tracking
- **HTTP Client:** httpx with async support

### Database Schema
```sql
CREATE TABLE webhook_endpoints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    url TEXT NOT NULL,
    secret TEXT NOT NULL,  -- For HMAC signing
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE webhook_deliveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    endpoint_id UUID NOT NULL REFERENCES webhook_endpoints(id),
    event_type VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    last_attempt_at TIMESTAMPTZ,
    next_attempt_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE delivery_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    delivery_id UUID NOT NULL REFERENCES webhook_deliveries(id),
    attempt_number INTEGER NOT NULL,
    status_code INTEGER,
    response_body TEXT,
    error_message TEXT,
    duration_ms INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_deliveries_status ON webhook_deliveries(status, next_attempt_at);
CREATE INDEX idx_deliveries_endpoint ON webhook_deliveries(endpoint_id);
```

### Data Models
```python
from pydantic import BaseModel
from datetime import datetime
from enum import Enum

class DeliveryStatus(str, Enum):
    PENDING = "pending"
    IN_PROGRESS = "in_progress"
    COMPLETED = "completed"
    FAILED = "failed"

class WebhookPayload(BaseModel):
    event_type: str
    data: dict
    timestamp: datetime
    delivery_id: str

class DeliveryAttempt(BaseModel):
    attempt_number: int
    status_code: int | None
    response_body: str | None
    error_message: str | None
    duration_ms: int
    created_at: datetime
```

### HMAC Signature Generation
```python
import hmac
import hashlib
import time

def sign_payload(payload: bytes, secret: str, timestamp: int) -> str:
    """Generate HMAC-SHA256 signature for webhook payload."""
    message = f"{timestamp}.{payload.decode('utf-8')}"
    signature = hmac.new(
        secret.encode('utf-8'),
        message.encode('utf-8'),
        hashlib.sha256
    ).hexdigest()
    return f"t={timestamp},v1={signature}"

def generate_headers(payload: bytes, secret: str) -> dict:
    timestamp = int(time.time())
    signature = sign_payload(payload, secret, timestamp)
    return {
        "Content-Type": "application/json",
        "X-Webhook-Signature": signature,
        "X-Webhook-Timestamp": str(timestamp),
    }
```

### Celery Task
```python
from celery import Celery
import httpx

celery = Celery('webhooks', broker='redis://localhost:6379/0')

RETRY_DELAYS = [60, 300, 1800, 7200, 86400]  # 1m, 5m, 30m, 2h, 24h

@celery.task(bind=True, max_retries=5)
def deliver_webhook(self, delivery_id: str):
    delivery = db.get_delivery(delivery_id)
    endpoint = db.get_endpoint(delivery.endpoint_id)

    if not endpoint.active:
        db.update_delivery(delivery_id, status=DeliveryStatus.FAILED)
        return

    payload = json.dumps({
        "event_type": delivery.event_type,
        "data": delivery.payload,
        "timestamp": delivery.created_at.isoformat(),
        "delivery_id": str(delivery_id),
    }).encode('utf-8')

    headers = generate_headers(payload, endpoint.secret)
    start = time.monotonic()

    try:
        with httpx.Client(timeout=30.0) as client:
            response = client.post(
                endpoint.url,
                content=payload,
                headers=headers,
            )

        duration_ms = int((time.monotonic() - start) * 1000)

        db.record_attempt(
            delivery_id=delivery_id,
            attempt_number=self.request.retries + 1,
            status_code=response.status_code,
            response_body=response.text[:1000],
            duration_ms=duration_ms,
        )

        if 200 <= response.status_code < 300:
            db.update_delivery(delivery_id, status=DeliveryStatus.COMPLETED)
            return

        # Retry on 5xx or specific 4xx codes
        if response.status_code >= 500 or response.status_code in [408, 429]:
            raise self.retry(
                countdown=RETRY_DELAYS[min(self.request.retries, len(RETRY_DELAYS) - 1)]
            )

        # Permanent failure for other 4xx
        db.update_delivery(delivery_id, status=DeliveryStatus.FAILED)

    except httpx.RequestError as e:
        duration_ms = int((time.monotonic() - start) * 1000)
        db.record_attempt(
            delivery_id=delivery_id,
            attempt_number=self.request.retries + 1,
            error_message=str(e),
            duration_ms=duration_ms,
        )
        raise self.retry(
            countdown=RETRY_DELAYS[min(self.request.retries, len(RETRY_DELAYS) - 1)]
        )

    except self.MaxRetriesExceededError:
        db.update_delivery(delivery_id, status=DeliveryStatus.FAILED)
```

### API Endpoints
```python
@router.post("/webhooks/{endpoint_id}/send")
async def send_webhook(endpoint_id: UUID, payload: WebhookPayload, db: Session = Depends()):
    endpoint = db.get(Endpoint, endpoint_id)
    if not endpoint:
        raise HTTPException(404, "Endpoint not found")

    delivery = Delivery(
        endpoint_id=endpoint_id,
        event_type=payload.event_type,
        payload=payload.data,
    )
    db.add(delivery)
    db.commit()

    deliver_webhook.delay(str(delivery.id))

    return {"delivery_id": str(delivery.id), "status": "queued"}

@router.get("/webhooks/deliveries/{delivery_id}")
async def get_delivery_status(delivery_id: UUID, db: Session = Depends()):
    delivery = db.get(Delivery, delivery_id)
    if not delivery:
        raise HTTPException(404)

    attempts = db.query(DeliveryAttempt).filter_by(delivery_id=delivery_id).all()

    return {
        "id": str(delivery.id),
        "status": delivery.status,
        "attempts": [a.dict() for a in attempts],
    }
```

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
