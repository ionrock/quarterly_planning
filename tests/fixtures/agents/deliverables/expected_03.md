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

#### Acceptance Criteria

1. **Gateway Configuration**
   - [ ] ApolloGateway initialized with supergraph SDL
   - [ ] Subgraph URLs configurable via environment variables
   - [ ] Health check endpoint at /health
   - [ ] Introspection disabled in production

2. **Subgraph Discovery**
   - [ ] Static subgraph list loaded from config
   - [ ] Each subgraph has name, URL, and optional headers
   - [ ] Failed subgraph doesn't crash gateway
   - [ ] Retry logic for transient failures (3 retries, exponential backoff)

3. **Query Resolution**
   - [ ] Queries spanning multiple subgraphs work
   - [ ] Entity references resolved correctly
   - [ ] Errors from subgraphs propagated with source
   - [ ] Partial results returned when possible

4. **Schema Composition**
   - [ ] Supergraph schema generated from subgraph schemas
   - [ ] Conflicting types detected at startup
   - [ ] Schema changes require gateway restart

#### Demo Script
```bash
# Start gateway
npm run start:gateway

# Query spanning multiple subgraphs
curl -X POST http://localhost:4000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ user(id: \"1\") { name orders { id total } } }"}'
# Expected: { "data": { "user": { "name": "John", "orders": [...] } } }

# Health check
curl http://localhost:4000/health
# Expected: { "status": "healthy", "subgraphs": { "users": "up", "orders": "up" } }

# Introspection (should fail in prod)
curl -X POST http://localhost:4000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ __schema { types { name } } }"}'
# Expected in prod: { "errors": [{ "message": "Introspection disabled" }] }
```

#### Test Requirements
- [ ] Integration test with mock subgraphs
- [ ] Test cross-subgraph query resolution
- [ ] Test subgraph failure handling
- [ ] Load test: 1000 req/s sustained for 60s

### Ticket 2: Authentication Middleware

**Summary:** Implement JWT validation and user context propagation.

**Definition of Done:** Protected queries require valid tokens.

#### Acceptance Criteria

1. **JWT Validation**
   - [ ] Authorization header parsed (Bearer token)
   - [ ] Token signature verified with public key
   - [ ] Token expiration checked
   - [ ] Invalid token returns 401 Unauthorized

2. **User Context**
   - [ ] Decoded user info added to GraphQL context
   - [ ] User ID, email, roles extracted from token
   - [ ] Context available to all resolvers
   - [ ] Context propagated to subgraphs via headers

3. **Protected Operations**
   - [ ] @authenticated directive marks protected fields
   - [ ] Protected fields return null without valid token
   - [ ] Error message indicates authentication required
   - [ ] Public fields remain accessible

4. **Token Refresh**
   - [ ] Near-expiry tokens trigger refresh hint in response
   - [ ] Refresh endpoint accepts refresh token
   - [ ] New access token returned

#### Demo Script
```bash
# Get token (from auth service)
TOKEN=$(curl -X POST http://localhost:4001/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "secret"}' | jq -r '.token')

# Authenticated query
curl -X POST http://localhost:4000/graphql \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"query": "{ me { email profile { address } } }"}'
# Expected: { "data": { "me": { "email": "user@example.com", "profile": {...} } } }

# Without token (protected field)
curl -X POST http://localhost:4000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ me { email } }"}'
# Expected: { "data": { "me": null }, "errors": [{ "message": "Authentication required" }] }

# Expired token
curl -X POST http://localhost:4000/graphql \
  -H "Authorization: Bearer expired.token.here" \
  -d '{"query": "{ me { email } }"}'
# Expected: HTTP 401, { "error": "Token expired" }
```

#### Test Requirements
- [ ] Unit tests for JWT validation logic
- [ ] Test expired token rejection
- [ ] Test invalid signature rejection
- [ ] Test context propagation to subgraphs
- [ ] Test public vs protected field access

### Ticket 3: Rate Limiting

**Summary:** Add per-user and per-IP rate limiting.

**Definition of Done:** Excessive requests are rejected with 429.

#### Acceptance Criteria

1. **Rate Limit Storage**
   - [ ] Redis used for rate limit counters
   - [ ] Sliding window algorithm (not fixed window)
   - [ ] TTL set on keys to prevent memory leak
   - [ ] Graceful degradation if Redis unavailable

2. **Per-IP Limiting**
   - [ ] Anonymous requests limited by IP
   - [ ] Default: 100 requests per minute
   - [ ] X-Forwarded-For header respected (configurable)
   - [ ] Localhost/internal IPs can be whitelisted

3. **Per-User Limiting**
   - [ ] Authenticated requests limited by user ID
   - [ ] Default: 1000 requests per minute
   - [ ] Different limits per user role (admin: unlimited)
   - [ ] User limits override IP limits

4. **Response Headers**
   - [ ] X-RateLimit-Limit: maximum requests allowed
   - [ ] X-RateLimit-Remaining: requests left in window
   - [ ] X-RateLimit-Reset: Unix timestamp when window resets
   - [ ] Retry-After header on 429 responses

5. **429 Response**
   - [ ] HTTP 429 Too Many Requests status
   - [ ] JSON body with error message and retry time
   - [ ] Request not processed (no subgraph calls)

#### Demo Script
```bash
# Check rate limit headers
curl -i -X POST http://localhost:4000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ __typename }"}'
# Headers: X-RateLimit-Limit: 100, X-RateLimit-Remaining: 99, X-RateLimit-Reset: 1706000000

# Exceed rate limit (run in loop)
for i in {1..101}; do
  curl -s -o /dev/null -w "%{http_code}\n" -X POST http://localhost:4000/graphql \
    -H "Content-Type: application/json" \
    -d '{"query": "{ __typename }"}'
done
# Expected: 100x 200, then 429

# Rate limited response
curl -i -X POST http://localhost:4000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ __typename }"}'
# Expected: HTTP 429, Retry-After: 45
# Body: { "error": "Rate limit exceeded", "retryAfter": 45 }
```

#### Test Requirements
- [ ] Unit tests for sliding window algorithm
- [ ] Integration test with Redis
- [ ] Test per-IP limiting
- [ ] Test per-user limiting
- [ ] Test rate limit header accuracy
- [ ] Test Redis failover behavior
