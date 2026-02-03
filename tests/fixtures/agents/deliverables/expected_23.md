---
id: "test-023"
title: "API Rate Limiter"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build an API rate limiting service as a sidecar or middleware. Supports multiple algorithms, distributed counting, and real-time analytics.

## Constraints

- Add less than 1ms latency
- Handle 100,000 requests per second

## Implementation Notes

- Rust for performance
- Redis for distributed state
- Token bucket and sliding window algorithms

## Review Notes

(none yet)

## Tickets

### Ticket 1: Rate Limit Algorithms

**Summary:** Implement token bucket and sliding window.

**Definition of Done:** Both algorithms enforce limits correctly.

#### Acceptance Criteria

1. **Token Bucket**
   - [ ] Configurable bucket size (max burst)
   - [ ] Configurable refill rate (tokens per second)
   - [ ] Atomic token consumption
   - [ ] Partial consumption (request N tokens)

2. **Sliding Window Log**
   - [ ] Track request timestamps
   - [ ] Count requests in sliding window
   - [ ] Precise rate limiting
   - [ ] Memory-efficient storage

3. **Sliding Window Counter**
   - [ ] Fixed window counters
   - [ ] Weighted average across windows
   - [ ] Less memory than log
   - [ ] Slightly less precise

4. **Fixed Window**
   - [ ] Simple counter per time window
   - [ ] Reset at window boundary
   - [ ] Lowest memory usage
   - [ ] Potential burst at boundary

5. **Algorithm Selection**
   - [ ] Configure algorithm per limit rule
   - [ ] Default: sliding window counter
   - [ ] Benchmarks comparing algorithms

#### Demo Script
```rust
use rate_limiter::{TokenBucket, SlidingWindow};

// Token bucket: 100 requests/minute with burst of 20
let bucket = TokenBucket::new(
    capacity: 20,           // Max burst
    refill_rate: 100.0/60.0 // ~1.67 tokens/second
);

// Check rate limit
let result = bucket.check("user-123");
match result {
    Allowed { remaining: 15 } => process_request(),
    Limited { retry_after: Duration::from_secs(2) } => return_429(),
}

// Sliding window: 1000 requests per minute
let window = SlidingWindow::new(
    max_requests: 1000,
    window_size: Duration::from_secs(60)
);

// Check and consume
let result = window.check("api-key-xyz");
// Returns: Allowed or Limited with retry_after

// Consume multiple tokens (batch request)
let result = bucket.check_n("user-123", 5); // Consume 5 tokens
```

```bash
# Compare algorithms (benchmark)
./rate-limiter benchmark --algorithm token-bucket
# Token Bucket: 2.1M checks/sec, avg latency: 0.48µs

./rate-limiter benchmark --algorithm sliding-window-log
# Sliding Window Log: 1.8M checks/sec, avg latency: 0.55µs

./rate-limiter benchmark --algorithm sliding-window-counter
# Sliding Window Counter: 2.0M checks/sec, avg latency: 0.50µs
```

#### Test Requirements
- [ ] Test token bucket enforcement
- [ ] Test sliding window accuracy
- [ ] Test burst handling
- [ ] Test edge cases (boundary conditions)
- [ ] Benchmark each algorithm
- [ ] Test memory usage under load

### Ticket 2: Distributed Counting

**Summary:** Coordinate limits across instances.

**Definition of Done:** Limits accurate within 1% across cluster.

#### Acceptance Criteria

1. **Redis Backend**
   - [ ] Lua scripts for atomic operations
   - [ ] Connection pooling
   - [ ] Automatic reconnection
   - [ ] Cluster mode support

2. **Synchronization Strategy**
   - [ ] Synchronous: check Redis on every request
   - [ ] Async: local counter with periodic sync
   - [ ] Hybrid: local cache with Redis fallback

3. **Consistency Guarantees**
   - [ ] Strong consistency option (slower)
   - [ ] Eventual consistency option (faster)
   - [ ] Configurable sync interval
   - [ ] Within 1% of true count

4. **Failure Handling**
   - [ ] Fallback to local-only on Redis failure
   - [ ] Configurable behavior: allow or deny
   - [ ] Health check endpoint
   - [ ] Metrics on Redis latency

5. **Key Design**
   - [ ] Key pattern: {prefix}:{identifier}:{window}
   - [ ] TTL matches window size
   - [ ] Sharding by identifier hash

#### Demo Script
```rust
use rate_limiter::{DistributedLimiter, RedisBackend};

// Create distributed limiter
let redis = RedisBackend::new("redis://localhost:6379")?;
let limiter = DistributedLimiter::new(redis, Config {
    sync_mode: SyncMode::Hybrid,
    local_cache_size: 10_000,
    sync_interval: Duration::from_millis(100),
});

// Check rate limit (coordinated across instances)
let result = limiter.check("user-123", &Rule {
    max_requests: 1000,
    window: Duration::from_secs(60),
}).await?;

// Result includes global count
// result.count: 523 (across all instances)
// result.remaining: 477
```

```bash
# Test distributed counting
# Start 3 instances
./rate-limiter serve --port 8001 &
./rate-limiter serve --port 8002 &
./rate-limiter serve --port 8003 &

# Send requests to different instances
for i in {1..100}; do
  curl -s "http://localhost:800$((i % 3 + 1))/check?key=user-123"
done

# All instances should show ~100 total requests
curl http://localhost:8001/stats?key=user-123
# {"count": 100, "limit": 1000, "remaining": 900}
```

#### Test Requirements
- [ ] Test Redis integration
- [ ] Test count accuracy across instances
- [ ] Test sync modes (sync, async, hybrid)
- [ ] Test Redis failure handling
- [ ] Test cluster mode
- [ ] Load test: 100k req/s distributed

### Ticket 3: Configuration and Analytics

**Summary:** Configure limits and view usage metrics.

**Definition of Done:** Limits configurable, metrics visible.

#### Acceptance Criteria

1. **Rule Configuration**
   - [ ] YAML/JSON configuration file
   - [ ] API for dynamic rule updates
   - [ ] Rules by: path, method, API key, user ID
   - [ ] Multiple rules with priority

2. **Rule Matching**
   - [ ] Path pattern matching: /api/v1/*
   - [ ] Method-specific limits
   - [ ] Header-based identification
   - [ ] Custom identifier extraction

3. **Response Headers**
   - [ ] X-RateLimit-Limit: total allowed
   - [ ] X-RateLimit-Remaining: requests left
   - [ ] X-RateLimit-Reset: window reset time
   - [ ] Retry-After: seconds until retry

4. **Prometheus Metrics**
   - [ ] requests_total{key, status}
   - [ ] requests_limited_total{key}
   - [ ] limit_utilization{key} (gauge)
   - [ ] check_latency_seconds

5. **Dashboard**
   - [ ] Real-time usage by key
   - [ ] Top rate-limited keys
   - [ ] Limit utilization heatmap
   - [ ] Latency percentiles

#### Demo Script
```yaml
# rate-limits.yaml
rules:
  - name: api-default
    match:
      path: /api/*
    limit:
      requests: 1000
      window: 1m
    algorithm: sliding-window-counter

  - name: api-authenticated
    match:
      path: /api/*
      header:
        X-API-Key: "*"
    limit:
      requests: 10000
      window: 1m
    identifier: "header:X-API-Key"

  - name: expensive-endpoint
    match:
      path: /api/v1/export
      method: POST
    limit:
      requests: 10
      window: 1h
    priority: 100  # Higher priority = checked first
```

```bash
# Check rate limit via API
curl -i http://localhost:8000/api/users \
  -H "X-API-Key: key-123"
# HTTP/1.1 200 OK
# X-RateLimit-Limit: 10000
# X-RateLimit-Remaining: 9950
# X-RateLimit-Reset: 1705320060

# When limited
curl -i http://localhost:8000/api/users
# HTTP/1.1 429 Too Many Requests
# X-RateLimit-Limit: 1000
# X-RateLimit-Remaining: 0
# X-RateLimit-Reset: 1705320060
# Retry-After: 45

# Update rule dynamically
curl -X PUT http://localhost:8000/admin/rules/api-default \
  -d '{"limit": {"requests": 2000, "window": "1m"}}'

# Prometheus metrics
curl http://localhost:8000/metrics
# rate_limiter_requests_total{key="anonymous",status="allowed"} 15000
# rate_limiter_requests_total{key="anonymous",status="limited"} 500
# rate_limiter_check_latency_seconds{quantile="0.99"} 0.0008

# Get top limited keys
curl http://localhost:8000/admin/stats/top-limited
# [{"key": "ip:1.2.3.4", "limited_count": 500}, ...]
```

#### Test Requirements
- [ ] Test rule configuration loading
- [ ] Test rule matching priority
- [ ] Test response headers accuracy
- [ ] Test Prometheus metrics
- [ ] Test dynamic rule updates
- [ ] Test dashboard API endpoints
