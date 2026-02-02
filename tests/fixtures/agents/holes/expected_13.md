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

### Identified Weaknesses

1. **No audit logging**: Who changed what flag and when? Critical for debugging and compliance.

2. **Polling delay not addressed**: What's the maximum time for flag changes to propagate?

3. **No fallback behavior**: What happens when the service is unreachable?

4. **Missing kill switch**: How to quickly disable a problematic flag across all services?

5. **Percentage rollout consistency**: Same user should consistently get same variant (sticky bucketing).

### Edge Cases

- What if a user has no ID (anonymous users)?
- How are percentage rollouts calculated deterministically?
- What if targeting rules conflict or overlap?
- How are deleted flags handled in running applications?
- What about flags with dependencies on other flags?
- Flag evaluation for users with incomplete attributes?

### Assumptions to Validate

- Is this replacing an existing feature flag system?
- What targeting attributes will be available (user ID, region, plan)?
- Do we need real-time updates (streaming) or is polling acceptable?
- Is multi-tenancy required (multiple teams/projects)?
- What's the expected flag count (tens, hundreds, thousands)?

### Potential Failures

- Cache invalidation bugs (stale flag values)
- Database overload during bulk flag updates
- SDK fails to connect (default values needed)
- Inconsistent evaluation across service instances
- Memory exhaustion from unbounded flag cache
- Thundering herd on service restart (all SDKs poll simultaneously)

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
