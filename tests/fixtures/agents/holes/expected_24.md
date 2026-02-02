---
id: "test-024"
title: "Distributed Lock Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Implement a distributed lock service for coordinating access to shared resources across multiple services. Provides mutual exclusion with automatic lock expiration. Built on Redis with a Go client library.

## Constraints

- Lock acquisition under 10ms
- Must handle Redis failover

## Implementation Notes

- Use Redis SET NX with expiration for lock acquisition
- Implement lock extension for long-running operations
- Provide blocking acquire with timeout
- Release locks with Lua script for atomicity

## Review Notes

### Identified Weaknesses

1. **Single Redis instance is not safe**: SET NX on single node can lose locks during failover; need Redlock algorithm for safety.

2. **Clock drift not addressed**: Lock expiration depends on time; clock skew between nodes causes issues.

3. **No fencing tokens**: Client may continue operating after lock expires; need fencing for correctness.

4. **Lock ownership verification missing**: Lua release script should verify caller owns lock before releasing.

5. **No lock contention metrics**: Hard to debug without visibility into wait times, failures.

### Edge Cases

- Process crashes while holding lock (relies on expiration)
- Network partition where client thinks it has lock but Redis has expired it
- GC pause causes lock to expire while client is frozen
- Lock name collisions between services
- Very long lock extension chains
- Releasing a lock you don't own (if ID verification fails)

### Assumptions to Validate

- Is Redis Cluster or Sentinel being used for HA?
- What's the acceptable failure mode (fail-open vs fail-closed)?
- Do we need read-write lock semantics?
- Is lock reentry (same owner acquiring twice) needed?
- What's the expected lock duration distribution?

### Potential Failures

- Split-brain during Redis failover (two holders of same lock)
- Extension fails but operation continues
- Client blocks forever if blocking acquire hangs
- Redis memory exhaustion from leaked locks
- Lua script timeout on overloaded Redis
- DNS caching preventing failover detection

## Tickets

### Ticket 1: Lock Acquisition

**Summary:** Implement acquire with SET NX and expiration.

**Definition of Done:** Locks are acquired atomically.

### Ticket 2: Lock Extension

**Summary:** Allow extending lock TTL for long operations.

**Definition of Done:** Locks can be extended without release.

### Ticket 3: Client Library

**Summary:** Package as reusable Go library.

**Definition of Done:** Library is documented and easy to use.
