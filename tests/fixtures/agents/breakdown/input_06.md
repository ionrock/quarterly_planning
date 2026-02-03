---
id: "test-006"
title: "OAuth2 Authentication Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Implement an OAuth2 authentication service that supports multiple providers (Google, GitHub, Microsoft). Handles token exchange, user creation, and session management.

## Constraints

- Tokens must be securely stored
- Support refresh token rotation

## Implementation Notes

- Use passport.js for OAuth strategies
- JWT for session tokens
- PostgreSQL for user storage

## Review Notes

(none yet)

## Tickets

### Ticket 1: OAuth Provider Setup

**Summary:** Configure OAuth providers and callbacks.

**Definition of Done:** Can initiate OAuth flow with all providers.

### Ticket 2: Token Management

**Summary:** Handle access and refresh tokens.

**Definition of Done:** Tokens are issued, validated, and refreshed.

### Ticket 3: User Management

**Summary:** Create and link user accounts.

**Definition of Done:** Users created on first login, linked on subsequent.
