---
id: "test-015"
title: "Rate Limiter Library"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a rate limiting library supporting multiple algorithms (token bucket, sliding window, fixed window). Provides both in-memory and Redis-backed implementations.

## Constraints

- Sub-millisecond evaluation
- Thread-safe for concurrent access

## Implementation Notes

- Written in Python
- Async support with asyncio
- Pluggable storage backends

## Review Notes

(none yet)

## Tickets

### Ticket 1: Algorithm Implementation

**Summary:** Implement rate limiting algorithms.

**Definition of Done:** All three algorithms work correctly.

#### Steps

1. **Create package structure**
   - Create ratelimit/ directory
   - Create __init__.py, algorithms.py
   - Verify: package importable

2. **Define RateLimitResult dataclass**
   - Include: allowed, remaining, reset_at, retry_after
   - Verify: dataclass works

3. **Define Algorithm base class**
   - Create abstract check(key, limit, window) -> RateLimitResult
   - Verify: base class compiles

4. **Implement fixed window algorithm**
   - Create FixedWindowAlgorithm class
   - Count requests in fixed time buckets
   - Reset count at bucket boundary
   - Verify: fixed window works

5. **Test fixed window edge cases**
   - Test at window boundary
   - Test window rollover
   - Verify: edge cases pass

6. **Implement sliding window log algorithm**
   - Create SlidingWindowLogAlgorithm class
   - Store timestamp of each request
   - Count requests in sliding window
   - Verify: sliding window accurate

7. **Test sliding window accuracy**
   - Compare to fixed window
   - Verify smoother rate limiting
   - Verify: no boundary spikes

8. **Implement token bucket algorithm**
   - Create TokenBucketAlgorithm class
   - Refill tokens at constant rate
   - Allow bursts up to bucket size
   - Verify: token bucket works

9. **Test token bucket burst**
   - Allow burst after idle
   - Refill after consumption
   - Verify: burst behavior correct

10. **Add algorithm factory**
    - Create get_algorithm(name) function
    - Map names to classes
    - Verify: factory works

### Ticket 2: Storage Backends

**Summary:** Create in-memory and Redis storage.

**Definition of Done:** Both backends pass all tests.

#### Steps

1. **Define Storage interface**
   - Create storage.py
   - Define abstract methods: get, set, increment, expire
   - Verify: interface compiles

2. **Implement MemoryStorage**
   - Create in-memory dict storage
   - Use threading.Lock for thread safety
   - Verify: concurrent access safe

3. **Add TTL support to MemoryStorage**
   - Track expiration times
   - Clean expired entries
   - Verify: entries expire

4. **Create background cleanup**
   - Periodically remove expired entries
   - Use daemon thread
   - Verify: memory doesn't grow

5. **Install Redis async client**
   - Add redis[hiredis] to dependencies
   - Verify: package installed

6. **Implement RedisStorage**
   - Create Redis-backed storage
   - Use Redis INCR for atomicity
   - Verify: Redis operations work

7. **Add Redis Lua scripts**
   - Use Lua for atomic check-and-set
   - Implement sliding window in Lua
   - Verify: atomic operations work

8. **Handle Redis connection errors**
   - Retry on connection failure
   - Fall back to allow on persistent failure
   - Verify: graceful degradation

9. **Create async storage interface**
   - Define AsyncStorage base class
   - Verify: interface compiles

10. **Implement AsyncRedisStorage**
    - Use aioredis for async operations
    - Verify: async operations work

11. **Write storage tests**
    - Test thread safety
    - Test expiration
    - Test concurrent access
    - Verify: all tests pass

### Ticket 3: Integration

**Summary:** Create middleware and decorators.

**Definition of Done:** Easy integration with web frameworks.

#### Steps

1. **Create RateLimiter class**
   - Accept algorithm and storage in constructor
   - Provide check(key, limit, window) method
   - Verify: limiter works

2. **Create rate_limit decorator**
   - Accept limit, window, key_func parameters
   - Raise RateLimitExceeded on limit
   - Verify: decorator works

3. **Create async decorator**
   - Handle async functions
   - Use async storage
   - Verify: async decorator works

4. **Create FastAPI middleware**
   - Create FastAPIRateLimitMiddleware
   - Check rate limit before request
   - Return 429 on limit exceeded
   - Verify: middleware works with FastAPI

5. **Create Flask extension**
   - Create FlaskRateLimiter extension
   - Register before_request handler
   - Verify: extension works with Flask

6. **Add rate limit headers**
   - X-RateLimit-Limit
   - X-RateLimit-Remaining
   - X-RateLimit-Reset
   - Verify: headers in response

7. **Create key extractors**
   - By IP address
   - By user ID (from auth)
   - By API key
   - Verify: extractors work

8. **Support custom key functions**
   - Accept callable for key generation
   - Pass request to callable
   - Verify: custom keys work

9. **Add configuration from env**
   - Read limits from environment
   - Support different limits per route
   - Verify: config works

10. **Write integration tests**
    - Test with FastAPI
    - Test with Flask
    - Verify: frameworks work

11. **Create usage documentation**
    - Document installation
    - Document configuration
    - Add examples for each framework
    - Verify: docs are complete

12. **Publish to PyPI**
    - Create setup.py
    - Build and upload
    - Verify: pip install works
