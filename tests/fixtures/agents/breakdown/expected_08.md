---
id: "test-008"
title: "GraphQL API Gateway"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a GraphQL API gateway that federates multiple microservices. Provides unified schema, authentication, and rate limiting.

## Constraints

- Query latency under 200ms
- Support schema stitching

## Implementation Notes

- Use Apollo Server with Federation
- Redis for caching
- JWT authentication

## Review Notes

(none yet)

## Tickets

### Ticket 1: Gateway Setup

**Summary:** Configure Apollo Gateway with subgraphs.

**Definition of Done:** Gateway routes queries to services.

#### Steps

1. **Create gateway project**
   - Run `npm init -y`
   - Install @apollo/gateway @apollo/server graphql
   - Verify: dependencies installed

2. **Create subgraph configuration**
   - Create src/subgraphs.ts
   - Define array of { name, url } for each service
   - Read URLs from environment variables
   - Verify: config loads correctly

3. **Initialize Apollo Gateway**
   - Create src/gateway.ts
   - Use IntrospectAndCompose for automatic schema composition
   - Pass subgraph list
   - Verify: gateway initializes

4. **Create Apollo Server**
   - Create src/server.ts
   - Pass gateway to ApolloServer
   - Verify: server starts without errors

5. **Add health check endpoint**
   - Add GET /health route
   - Check gateway and subgraph connectivity
   - Verify: health check returns status

6. **Configure supergraph polling**
   - Set pollIntervalInMs for schema updates
   - Default to 30 seconds in production
   - Verify: schema changes detected

7. **Handle subgraph errors**
   - Configure error handling for unreachable subgraphs
   - Return partial results when possible
   - Verify: graceful degradation works

8. **Add request logging**
   - Log query name, variables (sanitized), duration
   - Use structured logging format
   - Verify: queries logged

9. **Set up GraphQL playground**
   - Enable in development only
   - Disable introspection in production
   - Verify: playground accessible in dev

### Ticket 2: Authentication

**Summary:** Implement JWT auth and context propagation.

**Definition of Done:** Auth context available in all resolvers.

#### Steps

1. **Install JWT dependencies**
   - Run `npm install jsonwebtoken jwks-rsa`
   - Verify: packages installed

2. **Create JWT verification module**
   - Create src/auth/verify.ts
   - Support both symmetric (secret) and asymmetric (JWKS) verification
   - Verify: can verify valid tokens

3. **Extract token from request**
   - Check Authorization header for Bearer token
   - Support x-api-key header as alternative
   - Verify: token extracted from various formats

4. **Build context from token**
   - Create src/context.ts
   - Decode token, extract userId, roles, permissions
   - Return anonymous context if no token
   - Verify: context built correctly

5. **Add context to Apollo Server**
   - Pass context function to server config
   - Context receives { req, res }
   - Verify: context available in resolvers

6. **Propagate headers to subgraphs**
   - Create buildService custom function
   - Forward Authorization header to subgraphs
   - Add x-request-id for tracing
   - Verify: subgraphs receive headers

7. **Create auth directive**
   - Create @auth directive for schema
   - Check for authenticated user in context
   - Verify: unauthenticated requests blocked

8. **Create role directive**
   - Create @requireRole(role: String!) directive
   - Check user roles in context
   - Verify: unauthorized roles blocked

9. **Handle token expiration**
   - Return specific error code for expired tokens
   - Include refresh instruction in error
   - Verify: expired tokens handled gracefully

10. **Add API key authentication**
    - Support x-api-key for service-to-service
    - Store valid keys in Redis or database
    - Verify: API key auth works

### Ticket 3: Performance

**Summary:** Add caching and rate limiting.

**Definition of Done:** Responses cached, abuse prevented.

#### Steps

1. **Install caching dependencies**
   - Run `npm install @apollo/server-plugin-response-cache`
   - Run `npm install ioredis`
   - Verify: packages installed

2. **Configure Redis cache backend**
   - Create src/cache/redis.ts
   - Implement KeyValueCache interface for Apollo
   - Verify: cache operations work

3. **Enable response caching plugin**
   - Add responseCachePlugin to server
   - Pass Redis cache backend
   - Verify: responses cached

4. **Configure cache hints in schema**
   - Add @cacheControl directive to types
   - Set maxAge for different resources
   - Verify: cache headers set correctly

5. **Implement cache invalidation**
   - Create invalidateCache(pattern) function
   - Call on mutations that modify data
   - Verify: cached data cleared on mutation

6. **Install rate limiting dependencies**
   - Run `npm install graphql-rate-limit-directive`
   - Verify: package installed

7. **Create rate limit directive**
   - Create @rateLimit(max: Int!, window: String!) directive
   - Store counters in Redis
   - Verify: directive available in schema

8. **Apply rate limits to sensitive operations**
   - Add @rateLimit to mutations
   - Add @rateLimit to expensive queries
   - Verify: limits enforced

9. **Configure per-user rate limits**
   - Use userId from context as rate limit key
   - Fall back to IP for anonymous users
   - Verify: limits tracked per user

10. **Add query complexity analysis**
    - Install graphql-query-complexity
    - Calculate complexity score for queries
    - Reject queries exceeding maximum
    - Verify: complex queries rejected

11. **Add depth limiting**
    - Install graphql-depth-limit
    - Set maximum query depth (e.g., 10)
    - Verify: deeply nested queries rejected

12. **Monitor and alert on performance**
    - Track query latency histogram
    - Alert when p99 exceeds threshold
    - Verify: metrics exported
