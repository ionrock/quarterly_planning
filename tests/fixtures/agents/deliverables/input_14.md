---
id: "test-014"
title: "API Mocking Server"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a mock API server that simulates backend services for frontend development and testing. Supports OpenAPI spec import, dynamic responses, and request recording.

## Constraints

- Response latency under 10ms
- Support 1000 concurrent connections

## Implementation Notes

- Node.js with Fastify
- JSON Schema validation
- WebSocket support

## Review Notes

(none yet)

## Tickets

### Ticket 1: Mock Definition

**Summary:** Define mock endpoints with response templates.

**Definition of Done:** Mocks configurable via YAML/JSON.

### Ticket 2: Dynamic Responses

**Summary:** Generate responses with Faker and templates.

**Definition of Done:** Responses vary realistically per request.

### Ticket 3: Request Recording

**Summary:** Record and replay real API traffic.

**Definition of Done:** Traffic recorded and replayed accurately.
