---
id: "test-003"
title: "GraphQL API Gateway"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a GraphQL API gateway that federates multiple backend services. Supports schema stitching, authentication, and rate limiting. Written in TypeScript with Apollo Server.

## Constraints

- Must handle 1000 requests/second
- Sub-100ms latency for cached queries

## Implementation Notes

- Apollo Federation 2.0
- Redis for caching and rate limiting
- JWT authentication

## Review Notes

(none yet)

## Tickets

### Ticket 1: Federation Setup

**Summary:** Configure Apollo Federation with subgraph discovery.

**Definition of Done:** Gateway resolves queries across multiple subgraphs.

### Ticket 2: Authentication Middleware

**Summary:** Implement JWT validation and user context propagation.

**Definition of Done:** Protected queries require valid tokens.

### Ticket 3: Rate Limiting

**Summary:** Add per-user and per-IP rate limiting.

**Definition of Done:** Excessive requests are rejected with 429.
