---
id: "test-008"
title: "Log Aggregation Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a log aggregation service that collects, indexes, and searches application logs. Supports structured logging, full-text search, and alerting.

## Constraints

- Ingest 100,000 logs per second
- Search latency under 500ms

## Implementation Notes

- Rust for log ingestion
- ClickHouse for storage
- Vector for log shipping

## Review Notes

(none yet)

## Tickets

### Ticket 1: Log Ingestion

**Summary:** HTTP and TCP endpoints for log ingestion.

**Definition of Done:** Logs received and stored reliably.

#### Acceptance Criteria

1. **HTTP Endpoint**
   - [ ] POST /api/v1/logs accepts JSON array
   - [ ] Batch size up to 1000 logs per request
   - [ ] Returns 202 Accepted immediately
   - [ ] gzip compression supported
   - [ ] API key authentication

2. **TCP Endpoint**
   - [ ] TCP server on port 5514
   - [ ] Syslog format (RFC 5424) supported
   - [ ] JSON-lines format supported
   - [ ] Newline-delimited messages
   - [ ] Connection keepalive

3. **Log Schema**
   - [ ] timestamp (required): ISO 8601 or Unix epoch
   - [ ] level: trace, debug, info, warn, error, fatal
   - [ ] message: string (required)
   - [ ] service: string (source application)
   - [ ] Additional fields stored as JSON

4. **Processing Pipeline**
   - [ ] Parse and validate incoming logs
   - [ ] Enrich with ingest timestamp, source IP
   - [ ] Buffer in memory (configurable size)
   - [ ] Batch write to ClickHouse

5. **Reliability**
   - [ ] Write-ahead log for crash recovery
   - [ ] Backpressure when storage slow
   - [ ] Dead letter queue for unparseable logs
   - [ ] Metrics: ingestion rate, latency, errors

#### Demo Script
```bash
# HTTP ingestion
curl -X POST http://localhost:8080/api/v1/logs \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $API_KEY" \
  -d '[
    {"timestamp": "2024-01-15T10:00:00Z", "level": "info", "message": "Request processed", "service": "api", "request_id": "abc123"},
    {"timestamp": "2024-01-15T10:00:01Z", "level": "error", "message": "Database timeout", "service": "api", "duration_ms": 5000}
  ]'
# Response: 202 Accepted, {"ingested": 2}

# TCP ingestion (syslog)
echo '<14>1 2024-01-15T10:00:00Z myhost api - - - Request processed' | nc localhost 5514

# TCP ingestion (JSON-lines)
echo '{"timestamp": "2024-01-15T10:00:00Z", "level": "info", "message": "Hello"}' | nc localhost 5514

# Check ingestion metrics
curl http://localhost:8080/metrics
# logs_ingested_total{method="http"} 1000000
# logs_ingested_total{method="tcp"} 500000
# ingestion_latency_seconds{quantile="0.99"} 0.005
```

#### Test Requirements
- [ ] Unit tests for parsing (syslog, JSON)
- [ ] Integration test with ClickHouse
- [ ] Load test: 100,000 logs/second for 5 minutes
- [ ] Test crash recovery (kill and restart)
- [ ] Test backpressure behavior

### Ticket 2: Search API

**Summary:** Full-text search with filtering.

**Definition of Done:** Logs searchable by any field.

#### Acceptance Criteria

1. **Search Endpoint**
   - [ ] POST /api/v1/search accepts query
   - [ ] Returns matching logs with pagination
   - [ ] Response time under 500ms for typical queries
   - [ ] Query timeout configurable (default 30s)

2. **Query Language**
   - [ ] Full-text search on message field
   - [ ] Field filters: level:error, service:api
   - [ ] Time range: from, to parameters
   - [ ] Boolean operators: AND, OR, NOT
   - [ ] Wildcard support: message:*timeout*

3. **Filtering**
   - [ ] Filter by log level
   - [ ] Filter by service name
   - [ ] Filter by any custom field
   - [ ] Numeric comparisons: duration_ms:>1000

4. **Aggregations**
   - [ ] Count by level
   - [ ] Count by service
   - [ ] Histogram by time bucket
   - [ ] Top N values for any field

5. **Results**
   - [ ] Logs returned newest-first (configurable)
   - [ ] Pagination with cursor or offset
   - [ ] Highlight matching terms
   - [ ] Total count included

#### Demo Script
```bash
# Simple search
curl -X POST http://localhost:8080/api/v1/search \
  -H "Content-Type: application/json" \
  -d '{
    "query": "timeout",
    "from": "2024-01-15T00:00:00Z",
    "to": "2024-01-15T23:59:59Z",
    "limit": 100
  }'
# Response: {"logs": [...], "total": 1234, "took_ms": 45}

# Filtered search
curl -X POST http://localhost:8080/api/v1/search \
  -d '{
    "query": "level:error AND service:api",
    "from": "2024-01-15T00:00:00Z",
    "to": "2024-01-15T23:59:59Z"
  }'

# Aggregation
curl -X POST http://localhost:8080/api/v1/search \
  -d '{
    "query": "*",
    "from": "2024-01-15T00:00:00Z",
    "to": "2024-01-15T23:59:59Z",
    "aggregations": {
      "by_level": {"field": "level"},
      "by_hour": {"type": "histogram", "field": "timestamp", "interval": "1h"}
    }
  }'
# Response: {"aggregations": {"by_level": {"error": 100, "warn": 500, "info": 10000}, ...}}
```

#### Test Requirements
- [ ] Unit tests for query parser
- [ ] Integration tests with ClickHouse
- [ ] Test each query operator
- [ ] Test aggregations
- [ ] Benchmark: search 1B logs in under 500ms
- [ ] Test pagination correctness

### Ticket 3: Alerting

**Summary:** Trigger alerts based on log patterns.

**Definition of Done:** Alerts fire when conditions match.

#### Acceptance Criteria

1. **Alert Rules**
   - [ ] Define alert with query and threshold
   - [ ] Threshold types: count, rate, percentage
   - [ ] Evaluation interval configurable
   - [ ] Multiple alerts on same query

2. **Conditions**
   - [ ] Count > N in time window
   - [ ] Rate > N per second/minute
   - [ ] Percentage of errors > N%
   - [ ] New unique value in field

3. **Notifications**
   - [ ] Webhook notification (POST to URL)
   - [ ] Slack integration
   - [ ] PagerDuty integration
   - [ ] Email notification

4. **Alert State**
   - [ ] States: OK, Alerting, No Data
   - [ ] Hysteresis: require N consecutive matches to fire
   - [ ] Cooldown period between alerts
   - [ ] Alert history stored

5. **Alert Management**
   - [ ] Create/update/delete alerts via API
   - [ ] Mute alerts temporarily
   - [ ] Acknowledge firing alerts
   - [ ] Escalation policies

#### Demo Script
```bash
# Create alert
curl -X POST http://localhost:8080/api/v1/alerts \
  -H "Content-Type: application/json" \
  -d '{
    "name": "High Error Rate",
    "query": "level:error AND service:api",
    "condition": {
      "type": "count",
      "threshold": 100,
      "window": "5m"
    },
    "notifications": [
      {"type": "slack", "channel": "#alerts"},
      {"type": "pagerduty", "service_key": "xxx"}
    ]
  }'
# Response: 201, {"id": "alert-123", ...}

# List alerts
curl http://localhost:8080/api/v1/alerts
# Response: [{"id": "alert-123", "name": "High Error Rate", "state": "OK", ...}]

# Check alert status
curl http://localhost:8080/api/v1/alerts/alert-123
# Response: {"id": "alert-123", "state": "Alerting", "last_triggered": "...", "value": 150}

# Mute alert
curl -X POST http://localhost:8080/api/v1/alerts/alert-123/mute \
  -d '{"duration": "1h", "reason": "Deploying fix"}'

# Acknowledge alert
curl -X POST http://localhost:8080/api/v1/alerts/alert-123/acknowledge \
  -d '{"user": "oncall@example.com"}'
```

#### Test Requirements
- [ ] Unit tests for condition evaluation
- [ ] Integration test with mock notification endpoints
- [ ] Test alert state transitions
- [ ] Test hysteresis behavior
- [ ] Test cooldown period
- [ ] Test webhook delivery and retry
