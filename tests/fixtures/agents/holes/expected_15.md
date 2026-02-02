---
id: "test-015"
title: "Rate Limiter Library"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a rate limiter library supporting multiple algorithms (token bucket, sliding window, fixed window). Can be used for API rate limiting, both in-memory and with Redis backend. Written in Python.

## Constraints

- Thread-safe for concurrent access
- Sub-millisecond overhead per check

## Implementation Notes

- Abstract interface for different algorithms
- Redis backend for distributed rate limiting
- In-memory backend for single-instance use
- Return remaining quota and reset time

## Review Notes

### Identified Weaknesses

1. **Atomic operations in Redis**: Lua scripts needed for correctness; not mentioned.

2. **Clock synchronization**: Distributed systems need consistent time; clock drift handling?

3. **No burst handling**: Token bucket allows bursts, but what's the burst size configuration?

4. **Memory cleanup for in-memory backend**: Old keys need expiration to prevent memory leak.

5. **No async support mentioned**: Modern Python APIs often need asyncio compatibility.

### Edge Cases

- What happens at exactly the rate limit boundary?
- How are requests consuming multiple tokens handled?
- What if Redis is temporarily unavailable?
- Negative remaining quota possible?
- Very large windows (daily/monthly limits)?
- Time zone considerations for fixed windows?

### Assumptions to Validate

- Is this for HTTP middleware or general purpose?
- What's the expected key cardinality (per-user, per-IP, per-endpoint)?
- Should we support rate limit headers (X-RateLimit-*)?
- Is async/await support required?
- Do we need composite keys (user + endpoint)?

### Potential Failures

- Redis connection timeout during check (fail open or closed?)
- Integer overflow on very high limits
- Race conditions between check and consume
- Memory exhaustion from too many unique keys
- Clock jumps (NTP sync, leap seconds, DST)
- Redis cluster resharding during operation

## Tickets

### Ticket 1: Algorithm Implementations

**Summary:** Implement token bucket, sliding window, and fixed window algorithms.

**Definition of Done:** All algorithms correctly limit rates.

### Ticket 2: Redis Backend

**Summary:** Implement Redis-backed rate limiter for distributed use.

**Definition of Done:** Rate limits are shared across multiple instances.

### Ticket 3: API Design

**Summary:** Design clean API for checking and consuming quota.

**Definition of Done:** API is intuitive and well-documented.
