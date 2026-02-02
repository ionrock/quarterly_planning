---
id: "test-013"
title: "Feature Flag Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a feature flag service that allows toggling features without deployments. Supports percentage rollouts, user targeting, and A/B testing. Provides SDKs for server-side evaluation. Built with Go and PostgreSQL.

## Constraints

- Evaluation latency under 5ms
- Support 100,000+ evaluations per second

## Implementation Notes

- REST API for flag management
- PostgreSQL for flag configuration storage
- In-memory cache for fast evaluation
- SDK polls for updates periodically

## Review Notes

(none yet)

## Tickets

### Ticket 1: Flag Management API

**Summary:** CRUD API for creating and updating feature flags.

**Definition of Done:** Flags can be created, updated, and deleted via API.

### Ticket 2: Evaluation Engine

**Summary:** Implement flag evaluation with targeting rules.

**Definition of Done:** Flags evaluate correctly based on user attributes.

### Ticket 3: SDK Implementation

**Summary:** Create Go SDK for server-side flag evaluation.

**Definition of Done:** SDK returns flag values with low latency.
