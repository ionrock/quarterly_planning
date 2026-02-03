---
id: "test-019"
title: "Webhook Delivery System"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a webhook delivery system for sending event notifications to external endpoints. Supports retries, signature verification, and delivery tracking.

## Constraints

- Deliver 10,000 webhooks per minute
- 99.9% delivery success rate

## Implementation Notes

- Go service
- PostgreSQL for state
- Redis for rate limiting

## Review Notes

(none yet)

## Tickets

### Ticket 1: Webhook Registration

**Summary:** Register and manage webhook endpoints.

**Definition of Done:** Endpoints CRUD with validation.

#### Acceptance Criteria

1. **Endpoint Registration**
   - [ ] POST /webhooks creates new endpoint
   - [ ] Required: url, events (array of event types)
   - [ ] Optional: secret (for signatures), description
   - [ ] URL validated (HTTPS required in production)

2. **Event Types**
   - [ ] Subscribe to specific event types
   - [ ] Wildcard subscription (all events)
   - [ ] Event types validated against schema
   - [ ] List available event types via API

3. **Endpoint Management**
   - [ ] GET /webhooks lists all endpoints
   - [ ] GET /webhooks/{id} returns single endpoint
   - [ ] PUT /webhooks/{id} updates endpoint
   - [ ] DELETE /webhooks/{id} removes endpoint

4. **Secret Management**
   - [ ] Secret generated if not provided
   - [ ] Secret can be rotated
   - [ ] Secret never returned in API responses
   - [ ] Multiple secrets for rotation period

5. **Endpoint Testing**
   - [ ] POST /webhooks/{id}/test sends test event
   - [ ] Test includes sample payload
   - [ ] Returns delivery status immediately

#### Demo Script
```bash
# Register webhook
curl -X POST http://localhost:8000/api/webhooks \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "url": "https://myapp.com/webhooks/receive",
    "events": ["order.created", "order.updated"],
    "description": "Order notifications"
  }'
# Response:
# {
#   "id": "wh_abc123",
#   "url": "https://myapp.com/webhooks/receive",
#   "events": ["order.created", "order.updated"],
#   "secret": "whsec_xyz789...",  # Only shown once
#   "status": "active"
# }

# List webhooks
curl http://localhost:8000/api/webhooks
# Response: {"webhooks": [...], "total": 5}

# Test webhook
curl -X POST http://localhost:8000/api/webhooks/wh_abc123/test
# Response: {"status": "delivered", "response_code": 200, "latency_ms": 150}

# List available event types
curl http://localhost:8000/api/webhooks/event-types
# Response: ["order.created", "order.updated", "order.cancelled", "user.created", ...]

# Rotate secret
curl -X POST http://localhost:8000/api/webhooks/wh_abc123/rotate-secret
# Response: {"secret": "whsec_new123..."}
```

#### Test Requirements
- [ ] Test CRUD operations
- [ ] Test URL validation (HTTPS required)
- [ ] Test event type validation
- [ ] Test secret generation and rotation
- [ ] Test endpoint ping/test
- [ ] Test wildcard subscription

### Ticket 2: Delivery Engine

**Summary:** Send webhooks with retries and signatures.

**Definition of Done:** Webhooks delivered reliably.

#### Acceptance Criteria

1. **Webhook Dispatch**
   - [ ] HTTP POST to registered URL
   - [ ] JSON body with event data
   - [ ] Custom headers: X-Webhook-ID, X-Event-Type, X-Timestamp

2. **Request Signing**
   - [ ] HMAC-SHA256 signature
   - [ ] Signature in X-Signature-256 header
   - [ ] Signature covers timestamp + body
   - [ ] Verification code snippet in docs

3. **Retry Logic**
   - [ ] Retry on 5xx errors and timeouts
   - [ ] Exponential backoff: 1m, 5m, 30m, 2h, 24h
   - [ ] Max 5 retry attempts
   - [ ] Give up after 24 hours

4. **Delivery Guarantees**
   - [ ] At-least-once delivery
   - [ ] Idempotency key in payload
   - [ ] Ordered delivery per endpoint (optional)

5. **Rate Limiting**
   - [ ] Per-endpoint rate limit
   - [ ] Backoff on 429 responses
   - [ ] Respect Retry-After header
   - [ ] Global throughput limit

#### Demo Script
```bash
# Webhook payload delivered
POST https://myapp.com/webhooks/receive
Headers:
  Content-Type: application/json
  X-Webhook-ID: evt_12345
  X-Event-Type: order.created
  X-Timestamp: 1705320000
  X-Signature-256: sha256=abc123...

Body:
{
  "id": "evt_12345",
  "type": "order.created",
  "created_at": "2024-01-15T10:00:00Z",
  "data": {
    "order_id": "ord_789",
    "total": 99.99,
    "items": [...]
  },
  "idempotency_key": "evt_12345_attempt_1"
}

# Signature verification (Node.js)
const crypto = require('crypto');

function verifySignature(payload, signature, secret, timestamp) {
  const signedPayload = `${timestamp}.${payload}`;
  const expected = crypto
    .createHmac('sha256', secret)
    .update(signedPayload)
    .digest('hex');
  return `sha256=${expected}` === signature;
}

# Trigger event (internal API)
curl -X POST http://localhost:8000/internal/events \
  -d '{
    "type": "order.created",
    "data": {"order_id": "ord_789", "total": 99.99}
  }'
# Response: {"event_id": "evt_12345", "deliveries_queued": 3}
```

#### Test Requirements
- [ ] Test successful delivery
- [ ] Test signature verification
- [ ] Test retry on 5xx errors
- [ ] Test retry on timeout
- [ ] Test backoff timing
- [ ] Test rate limiting
- [ ] Load test: 10,000 webhooks/minute

### Ticket 3: Monitoring Dashboard

**Summary:** Track delivery status and debug failures.

**Definition of Done:** Dashboard shows delivery metrics.

#### Acceptance Criteria

1. **Delivery Logs**
   - [ ] List all delivery attempts
   - [ ] Filter by: endpoint, event type, status, date range
   - [ ] Show: timestamp, status, response code, latency
   - [ ] Pagination for large result sets

2. **Delivery Details**
   - [ ] Full request: headers, body
   - [ ] Full response: status, headers, body
   - [ ] All retry attempts
   - [ ] Timeline of delivery attempts

3. **Metrics**
   - [ ] Success rate per endpoint
   - [ ] Average latency per endpoint
   - [ ] Delivery volume over time
   - [ ] Failure rate by error type

4. **Alerting**
   - [ ] Alert on high failure rate
   - [ ] Alert on endpoint consistently failing
   - [ ] Auto-disable endpoint after repeated failures
   - [ ] Email notification on disable

5. **Manual Actions**
   - [ ] Retry failed delivery manually
   - [ ] Resend any past event
   - [ ] Disable/enable endpoint
   - [ ] View/copy request for debugging

#### Demo Script
```bash
# List delivery attempts
curl "http://localhost:8000/api/webhooks/wh_abc123/deliveries?status=failed"
# Response:
# {
#   "deliveries": [
#     {
#       "id": "del_123",
#       "event_id": "evt_456",
#       "event_type": "order.created",
#       "status": "failed",
#       "attempts": 5,
#       "last_attempt_at": "2024-01-15T12:00:00Z",
#       "last_error": "Connection timeout"
#     }
#   ],
#   "total": 15
# }

# Get delivery details
curl http://localhost:8000/api/deliveries/del_123
# Response: Full request/response details for all attempts

# Retry delivery
curl -X POST http://localhost:8000/api/deliveries/del_123/retry
# Response: {"status": "queued"}

# Get endpoint metrics
curl http://localhost:8000/api/webhooks/wh_abc123/metrics
# Response:
# {
#   "success_rate": 0.95,
#   "avg_latency_ms": 230,
#   "deliveries_24h": 1500,
#   "failures_24h": 75,
#   "failure_reasons": {
#     "timeout": 50,
#     "5xx": 20,
#     "connection_refused": 5
#   }
# }
```

#### Test Requirements
- [ ] Test delivery log listing
- [ ] Test delivery detail retrieval
- [ ] Test manual retry
- [ ] Test metrics accuracy
- [ ] Test auto-disable on failure threshold
- [ ] Test alert triggering
- [ ] End-to-end dashboard test
