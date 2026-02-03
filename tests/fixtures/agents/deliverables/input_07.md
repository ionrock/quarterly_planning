---
id: "test-007"
title: "Feature Flag Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a feature flag service for gradual rollouts. Supports percentage-based rollouts, user targeting, and A/B testing. Provides SDKs for multiple languages.

## Constraints

- Flag evaluation under 5ms (p99)
- 99.99% availability

## Implementation Notes

- Go backend service
- PostgreSQL for flag storage
- Local caching in SDKs

## Review Notes

(none yet)

## Tickets

### Ticket 1: Flag Evaluation Engine

**Summary:** Implement flag evaluation with targeting rules.

**Definition of Done:** Flags evaluate correctly for all rule types.

### Ticket 2: Admin API

**Summary:** CRUD API for managing flags.

**Definition of Done:** Flags can be created, updated, and deleted.

### Ticket 3: Client SDK (JavaScript)

**Summary:** JavaScript SDK for flag evaluation.

**Definition of Done:** SDK evaluates flags with local caching.
