---
id: "test-003"
title: "Redis Caching Layer"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Implement a Redis-based caching layer for an existing web application to improve response times. The cache should handle session data, API responses, and computed values with appropriate TTLs.

## Constraints

- Must not break existing functionality
- Cache invalidation must be reliable

## Implementation Notes

- Use Redis 7.x with cluster support
- Implement cache-aside pattern
- Add monitoring for cache hit rates

## Review Notes

(none yet)

## Tickets

### Ticket 1: Redis Setup

**Summary:** Configure Redis connection and basic operations.

**Definition of Done:** Application connects to Redis and can get/set values.

#### Steps

1. **Add Redis client dependency**
   - Add ioredis package to package.json
   - Run npm install
   - Verify: package-lock.json updated

2. **Create Redis configuration module**
   - Create src/cache/config.ts
   - Read REDIS_URL from environment (default: redis://localhost:6379)
   - Export configuration object with host, port, password, db
   - Verify: config loads correctly from env

3. **Create Redis client singleton**
   - Create src/cache/client.ts
   - Initialize ioredis client with config
   - Handle connection events (connect, error, close)
   - Verify: client connects on import

4. **Add connection health check**
   - Create ping() function that calls Redis PING
   - Add to application health check endpoint
   - Verify: /health returns redis: "ok" when connected

5. **Create basic cache wrapper**
   - Create src/cache/index.ts with Cache class
   - Implement get(key: string): Promise<string | null>
   - Implement set(key: string, value: string, ttl?: number): Promise<void>
   - Verify: set then get returns same value

6. **Add JSON serialization helpers**
   - Add getJSON<T>(key): Promise<T | null> method
   - Add setJSON<T>(key, value: T, ttl?): Promise<void> method
   - Use JSON.stringify/parse internally
   - Verify: can store and retrieve objects

7. **Implement delete operation**
   - Add del(key: string): Promise<boolean> method
   - Return true if key existed, false otherwise
   - Verify: deleted key returns null on get

8. **Add key prefix support**
   - Accept optional prefix in Cache constructor
   - Prepend prefix to all keys automatically
   - Verify: keys stored with prefix in Redis

### Ticket 2: Caching Strategy

**Summary:** Implement caching patterns for different data types.

**Definition of Done:** Sessions, API responses, and computed values are cached.

#### Steps

1. **Create session cache module**
   - Create src/cache/session.ts
   - Use prefix "session:" for all session keys
   - Set TTL to 24 hours (86400 seconds)
   - Verify: session data stored with correct prefix and TTL

2. **Implement session store interface**
   - Create get(sessionId): Promise<SessionData | null>
   - Create set(sessionId, data): Promise<void>
   - Create destroy(sessionId): Promise<void>
   - Verify: sessions can be created, read, and destroyed

3. **Integrate session cache with auth middleware**
   - Modify auth middleware to use session cache
   - Fall back to database if cache miss
   - Populate cache on database read
   - Verify: second request hits cache (check logs/metrics)

4. **Create API response cache module**
   - Create src/cache/api.ts
   - Use prefix "api:" for all API cache keys
   - Generate cache key from request method + path + query params
   - Verify: cache key generation is deterministic

5. **Create cache middleware for routes**
   - Create src/middleware/cacheResponse.ts
   - Check cache before handler, return cached if hit
   - Cache response after handler with configurable TTL
   - Verify: second identical request returns cached response

6. **Apply cache middleware to read-only endpoints**
   - Add middleware to GET /api/products
   - Add middleware to GET /api/categories
   - Set TTL to 5 minutes for product data
   - Verify: responses cached and served from Redis

7. **Create computed values cache module**
   - Create src/cache/computed.ts
   - Use prefix "computed:" for computed values
   - Implement memoize<T>(key, compute: () => Promise<T>, ttl): Promise<T>
   - Verify: expensive computation only runs on cache miss

8. **Cache expensive database aggregations**
   - Wrap dashboard stats query with memoize
   - Wrap report generation with memoize
   - Set TTL based on data freshness requirements
   - Verify: dashboard loads faster on second view

### Ticket 3: Cache Invalidation

**Summary:** Implement reliable cache invalidation.

**Definition of Done:** Cache updates when underlying data changes.

#### Steps

1. **Create invalidation helper functions**
   - Create src/cache/invalidate.ts
   - Implement invalidateKey(key: string): Promise<void>
   - Implement invalidatePattern(pattern: string): Promise<number> using SCAN + DEL
   - Verify: specific keys and patterns can be invalidated

2. **Add cache tags support**
   - Create tag-to-keys mapping in Redis SET
   - When caching, add key to tag sets: cache:tag:{tagName}
   - Implement invalidateTag(tag: string) to delete all tagged keys
   - Verify: invalidating tag clears all associated keys

3. **Implement write-through invalidation for products**
   - In product.update(), call invalidateTag('products')
   - In product.create(), call invalidateTag('products')
   - In product.delete(), call invalidateKey(`api:GET:/products/${id}`)
   - Verify: product changes clear relevant cache entries

4. **Implement write-through invalidation for sessions**
   - In user.logout(), call sessionCache.destroy(sessionId)
   - In user.updatePassword(), invalidate all user sessions
   - Verify: logout immediately invalidates session

5. **Add cache versioning for schema changes**
   - Include version number in cache key prefix
   - Increment version when cached data structure changes
   - Verify: old cached data not returned after version bump

6. **Create cache invalidation events**
   - Emit 'cache:invalidate' event when invalidating
   - Log invalidation events for debugging
   - Verify: events logged with key/pattern info

7. **Add TTL safety net**
   - Ensure all cached values have maximum TTL (24 hours)
   - Prevent stale data even if invalidation fails
   - Verify: no keys without expiration in Redis (DEBUG OBJECT)

8. **Create cache warmup script**
   - Create scripts/warm-cache.ts
   - Pre-populate frequently accessed data on deploy
   - Verify: script runs successfully and populates cache

9. **Add cache metrics collection**
   - Track hits, misses, and invalidations
   - Export metrics to monitoring system
   - Verify: metrics visible in dashboard

10. **Document cache invalidation patterns**
    - Document which writes invalidate which cache keys
    - Add comments in code linking writes to invalidations
    - Verify: README includes cache strategy documentation
