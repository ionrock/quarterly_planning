---
id: "test-003"
title: "Redis Caching Layer"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Add a Redis caching layer to our existing e-commerce API to improve response times for product catalog queries. The cache will store frequently accessed product data and invalidate on updates.

## Constraints

- Must not break existing API contracts
- Redis cluster already available in staging/production

## Implementation Notes

### Technology Stack
- **Cache Client:** redis-py 5.x with connection pooling
- **Serialization:** MessagePack (faster and smaller than JSON)
- **Monitoring:** Redis metrics exposed via Prometheus client

### Cache Key Schema
```
product:{product_id}              - Single product details
products:list:{hash}              - Product listing (hash of query params)
products:category:{category_id}   - Products by category
products:invalidation:lock        - Distributed lock for cache warming
```

### Key Generation
```python
def cache_key_for_product(product_id: str) -> str:
    return f"product:{product_id}"

def cache_key_for_listing(params: ListingParams) -> str:
    # Deterministic hash of query parameters
    normalized = json.dumps(asdict(params), sort_keys=True)
    hash_val = hashlib.sha256(normalized.encode()).hexdigest()[:16]
    return f"products:list:{hash_val}"
```

### Cache-Aside Pattern Implementation
```python
async def get_product(product_id: str) -> Product:
    cache_key = cache_key_for_product(product_id)

    # Try cache first
    cached = await redis.get(cache_key)
    if cached:
        return msgpack.unpackb(cached, raw=False)

    # Cache miss - fetch from DB
    product = await db.fetch_product(product_id)
    if product:
        await redis.setex(
            cache_key,
            timedelta(minutes=5),
            msgpack.packb(asdict(product))
        )

    return product
```

### Invalidation Strategy
```python
async def invalidate_product(product_id: str) -> None:
    """Invalidate single product and related listings."""
    pipe = redis.pipeline()

    # Delete product cache
    pipe.delete(cache_key_for_product(product_id))

    # Delete all listing caches (use pattern matching sparingly)
    # Better: track which listings contain this product
    listing_keys = await redis.keys("products:list:*")
    if listing_keys:
        pipe.delete(*listing_keys)

    await pipe.execute()

async def invalidate_category(category_id: str) -> None:
    """Invalidate all products in a category."""
    await redis.delete(f"products:category:{category_id}")
```

### Connection Pool Configuration
```python
redis_pool = redis.ConnectionPool(
    host=settings.REDIS_HOST,
    port=settings.REDIS_PORT,
    db=0,
    max_connections=50,
    socket_timeout=5.0,
    socket_connect_timeout=5.0,
    retry_on_timeout=True,
    health_check_interval=30,
)
```

### Cache Stampede Prevention
```python
async def get_with_lock(key: str, fetch_fn: Callable) -> Any:
    """Prevent cache stampede using distributed lock."""
    cached = await redis.get(key)
    if cached:
        return msgpack.unpackb(cached)

    lock_key = f"{key}:lock"
    lock = redis.lock(lock_key, timeout=10, blocking_timeout=5)

    if await lock.acquire(blocking=True):
        try:
            # Double-check after acquiring lock
            cached = await redis.get(key)
            if cached:
                return msgpack.unpackb(cached)

            # Fetch and cache
            data = await fetch_fn()
            await redis.setex(key, TTL, msgpack.packb(data))
            return data
        finally:
            await lock.release()
    else:
        # Another process is fetching, wait and retry
        await asyncio.sleep(0.1)
        return await get_with_lock(key, fetch_fn)
```

### Graceful Degradation
```python
async def get_product_with_fallback(product_id: str) -> Product:
    try:
        return await get_product(product_id)
    except redis.RedisError as e:
        logger.warning(f"Redis error, falling back to DB: {e}")
        metrics.cache_errors.inc()
        return await db.fetch_product(product_id)
```

### Metrics
- `cache_hits_total` - Counter of cache hits
- `cache_misses_total` - Counter of cache misses
- `cache_latency_seconds` - Histogram of cache operation latency
- `cache_errors_total` - Counter of Redis errors

## Review Notes

(none yet)

## Tickets

### Ticket 1: Redis Client Setup

**Summary:** Add Redis client configuration and connection pooling.

**Definition of Done:** Application connects to Redis successfully.

### Ticket 2: Cache Read-Through

**Summary:** Implement cache lookup before database queries for product endpoints.

**Definition of Done:** Cache hits return data without database query.

### Ticket 3: Cache Invalidation

**Summary:** Invalidate relevant cache entries when products are modified.

**Definition of Done:** Updates are reflected immediately in subsequent reads.
