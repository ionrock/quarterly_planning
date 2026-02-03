---
id: "test-009"
title: "OAuth2 Authorization Server"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build an OAuth2 authorization server implementing RFC 6749. Supports authorization code, client credentials, and refresh token grants. Includes user consent UI.

## Constraints

- Must pass OAuth2 conformance tests
- Token generation under 50ms

## Implementation Notes

- Node.js with Express
- PostgreSQL for persistence
- JWT for access tokens

## Review Notes

(none yet)

## Tickets

### Ticket 1: Authorization Code Flow

**Summary:** Implement authorization code grant with PKCE.

**Definition of Done:** Code flow works with standard OAuth2 clients.

### Ticket 2: Token Management

**Summary:** Issue, validate, and revoke tokens.

**Definition of Done:** All token operations work correctly.

### Ticket 3: Consent UI

**Summary:** User-facing consent and login pages.

**Definition of Done:** Users can authorize or deny access requests.
