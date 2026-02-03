---
id: "test-024"
title: "Caching Layer Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a caching layer that sits between applications and databases. Supports read-through, write-through, and cache invalidation strategies.

## Constraints

- Cache hit latency under 1ms
- 99% cache hit rate for hot data

## Implementation Notes

- Go service
- Redis and Memcached backends
- Consistent hashing for sharding

## Review Notes

(none yet)

## Tickets

### Ticket 1: Cache Operations

**Summary:** Get, set, delete with TTL support.

**Definition of Done:** Basic cache operations work correctly.

### Ticket 2: Caching Strategies

**Summary:** Read-through, write-through, write-behind.

**Definition of Done:** All strategies work correctly.

### Ticket 3: Cache Invalidation

**Summary:** Invalidate by key, pattern, or tag.

**Definition of Done:** Invalid data evicted promptly.
