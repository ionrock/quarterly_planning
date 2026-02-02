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

(none yet)

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
