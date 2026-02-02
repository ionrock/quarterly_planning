---
id: "test-006"
title: "OAuth2 Authentication Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Implement an OAuth2 authentication service that supports Google and GitHub login. Users can authenticate via third-party providers and receive JWT tokens for API access. Built with Python/FastAPI.

## Constraints

- Must comply with OAuth2 specification
- Tokens expire after 1 hour

## Implementation Notes

- Use authlib for OAuth2 client implementation
- Store user profiles in PostgreSQL
- Generate JWT tokens with user claims
- Refresh token support for extended sessions

## Review Notes

### Identified Weaknesses

1. **Secret management not addressed**: Where are OAuth client secrets and JWT signing keys stored?

2. **No logout/token revocation**: How do users invalidate sessions? JWTs are stateless.

3. **Missing CSRF protection**: OAuth flows are vulnerable to CSRF without state parameters.

4. **No account linking strategy**: What if user has both Google and GitHub with same email?

5. **Refresh token security unclear**: Where are refresh tokens stored? Rotation policy?

### Edge Cases

- What if a user's email changes on the OAuth provider?
- How is a user handled if their OAuth account is deleted/disabled?
- What claims should be included in the JWT? Roles? Permissions?
- What if the OAuth provider is temporarily unavailable?
- How are users with private email on GitHub handled?
- What happens during token refresh if the refresh token is expired?

### Assumptions to Validate

- Is this service standalone or integrated into existing auth infrastructure?
- Are there existing users that need to be migrated/linked?
- What scopes do we need from each provider?
- Is email verification required, or do we trust provider verification?
- Do we need to support additional providers in the future (Apple, Microsoft)?

### Potential Failures

- OAuth state mismatch (expired or tampered)
- JWT signing key rotation without invalidating old tokens
- Database constraint violation on duplicate email
- Clock skew issues with JWT expiration
- Rate limiting by OAuth providers during high traffic
- Callback URL misconfiguration between environments

## Tickets

### Ticket 1: OAuth2 Client Setup

**Summary:** Configure OAuth2 clients for Google and GitHub.

**Definition of Done:** Redirect URLs work and tokens are received.

### Ticket 2: User Management

**Summary:** Store and retrieve user profiles from database.

**Definition of Done:** Users are created on first login and retrieved on subsequent logins.

### Ticket 3: JWT Token Generation

**Summary:** Generate and validate JWT tokens.

**Definition of Done:** Tokens are correctly signed and validated.
