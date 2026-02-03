---
id: "test-024"
title: "Caching Layer Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a caching layer that sits between applications and databases. Supports read-through, write-through, and cache invalidation strategies.

## Constraints

- Cache hit latency under 1ms
- 99% cache hit rate for hot data

## Implementation Notes

- Go service
- Redis and Memcached backends
- Consistent hashing for sharding

## Review Notes

(none yet)

## Tickets

### Ticket 1: Cache Operations

**Summary:** Get, set, delete with TTL support.

**Definition of Done:** Basic cache operations work correctly.

#### Acceptance Criteria

1. **Get Operation**
   - [ ] Get single key
   - [ ] Get multiple keys (batch)
   - [ ] Return nil for missing keys
   - [ ] Return with metadata (TTL remaining)

2. **Set Operation**
   - [ ] Set key with value
   - [ ] Set with TTL (expiration)
   - [ ] Set if not exists (NX)
   - [ ] Set if exists (XX)

3. **Delete Operation**
   - [ ] Delete single key
   - [ ] Delete multiple keys (batch)
   - [ ] Return success even if key missing

4. **TTL Management**
   - [ ] Default TTL configurable
   - [ ] Per-key TTL override
   - [ ] Extend TTL on access (optional)
   - [ ] No TTL (permanent until evicted)

5. **Serialization**
   - [ ] JSON serialization by default
   - [ ] MessagePack for efficiency
   - [ ] Protocol Buffers support
   - [ ] Compression for large values

#### Demo Script
```go
import "github.com/company/cache"

client := cache.NewClient("localhost:6379")

// Set with TTL
err := client.Set(ctx, "user:123", user, cache.WithTTL(time.Hour))

// Get
var user User
found, err := client.Get(ctx, "user:123", &user)
if !found {
    // Cache miss
}

// Batch get
keys := []string{"user:1", "user:2", "user:3"}
results, err := client.GetMany(ctx, keys)
// results: map[string]interface{}

// Set if not exists
set, err := client.SetNX(ctx, "lock:resource", "owner-1", cache.WithTTL(time.Minute))
if set {
    // Lock acquired
}

// Delete
err := client.Delete(ctx, "user:123")

// Get with metadata
value, meta, err := client.GetWithMeta(ctx, "user:123")
// meta.TTL: time remaining
// meta.Size: value size in bytes
```

```bash
# HTTP API
# Set value
curl -X PUT http://localhost:8000/cache/user:123 \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "email": "alice@example.com"}' \
  -H "X-Cache-TTL: 3600"
# Response: 201 Created

# Get value
curl http://localhost:8000/cache/user:123
# Response: {"name": "Alice", ...}
# Headers: X-Cache-Hit: true, X-Cache-TTL-Remaining: 3540

# Delete value
curl -X DELETE http://localhost:8000/cache/user:123
# Response: 204 No Content
```

#### Test Requirements
- [ ] Test get/set/delete operations
- [ ] Test TTL expiration
- [ ] Test conditional set (NX, XX)
- [ ] Test batch operations
- [ ] Test serialization formats
- [ ] Benchmark: latency < 1ms

### Ticket 2: Caching Strategies

**Summary:** Read-through, write-through, write-behind.

**Definition of Done:** All strategies work correctly.

#### Acceptance Criteria

1. **Read-Through**
   - [ ] On cache miss, fetch from origin
   - [ ] Populate cache with fetched value
   - [ ] Configurable origin function
   - [ ] Single flight (prevent thundering herd)

2. **Write-Through**
   - [ ] Write to cache and origin synchronously
   - [ ] Cache update only if origin succeeds
   - [ ] Atomic operation
   - [ ] Rollback on failure

3. **Write-Behind (Write-Back)**
   - [ ] Write to cache immediately
   - [ ] Async write to origin
   - [ ] Batching of writes
   - [ ] Retry on origin failure

4. **Cache-Aside**
   - [ ] Application manages cache
   - [ ] No automatic population
   - [ ] Explicit invalidation

5. **Refresh-Ahead**
   - [ ] Proactively refresh before expiry
   - [ ] Refresh when TTL < threshold
   - [ ] Background refresh
   - [ ] Never serve stale data

#### Demo Script
```go
// Read-through cache
cache := cache.NewReadThrough(redis, cache.ReadThroughConfig{
    Origin: func(ctx context.Context, key string) (interface{}, error) {
        // Fetch from database
        return db.GetUser(ctx, key)
    },
    TTL: time.Hour,
    SingleFlight: true, // Prevent thundering herd
})

user, err := cache.Get(ctx, "user:123")
// If miss: fetches from DB, populates cache, returns value

// Write-through cache
cache := cache.NewWriteThrough(redis, cache.WriteThroughConfig{
    Origin: func(ctx context.Context, key string, value interface{}) error {
        return db.SaveUser(ctx, key, value)
    },
})

err := cache.Set(ctx, "user:123", user)
// Writes to both cache and database atomically

// Write-behind cache
cache := cache.NewWriteBehind(redis, cache.WriteBehindConfig{
    Origin: db.SaveUser,
    BatchSize: 100,
    FlushInterval: time.Second,
})

err := cache.Set(ctx, "user:123", user)
// Writes to cache immediately, database async
```

```bash
# Configure strategy via API
curl -X POST http://localhost:8000/admin/strategy \
  -d '{
    "pattern": "user:*",
    "strategy": "read-through",
    "origin": {"type": "http", "url": "http://user-service/users/{key}"},
    "ttl": 3600
  }'

# Cache will auto-populate on miss
curl http://localhost:8000/cache/user:123
# Cache miss -> fetches from user-service -> returns and caches
```

#### Test Requirements
- [ ] Test read-through population
- [ ] Test write-through consistency
- [ ] Test write-behind batching
- [ ] Test single flight (thundering herd)
- [ ] Test origin failure handling
- [ ] Test refresh-ahead timing

### Ticket 3: Cache Invalidation

**Summary:** Invalidate by key, pattern, or tag.

**Definition of Done:** Invalid data evicted promptly.

#### Acceptance Criteria

1. **Key Invalidation**
   - [ ] Delete specific key
   - [ ] Delete multiple keys
   - [ ] Broadcast to all instances

2. **Pattern Invalidation**
   - [ ] Delete by glob pattern: user:*
   - [ ] Delete by prefix: user:
   - [ ] Scan-based deletion (non-blocking)
   - [ ] Progress tracking for large deletions

3. **Tag-Based Invalidation**
   - [ ] Associate keys with tags
   - [ ] Delete all keys with tag
   - [ ] Multiple tags per key
   - [ ] Example: tag "user:123" on all related caches

4. **Event-Driven Invalidation**
   - [ ] Webhook for database changes
   - [ ] Kafka consumer for CDC events
   - [ ] Debezium integration
   - [ ] Automatic key mapping

5. **Invalidation Propagation**
   - [ ] Pub/sub for multi-instance coordination
   - [ ] Eventual consistency guarantee
   - [ ] Latency monitoring
   - [ ] Propagation within 100ms

#### Demo Script
```go
// Tag keys on set
cache.Set(ctx, "user:123:profile", profile,
    cache.WithTags("user:123", "profiles"))
cache.Set(ctx, "user:123:orders", orders,
    cache.WithTags("user:123", "orders"))

// Invalidate by tag
cache.InvalidateByTag(ctx, "user:123")
// Deletes both user:123:profile and user:123:orders

// Pattern invalidation
cache.InvalidateByPattern(ctx, "user:123:*")

// Event-driven invalidation
cache.OnDatabaseChange(func(event DBEvent) {
    switch event.Table {
    case "users":
        cache.InvalidateByTag(ctx, fmt.Sprintf("user:%d", event.ID))
    }
})
```

```bash
# Delete by key
curl -X DELETE http://localhost:8000/cache/user:123

# Delete by pattern
curl -X DELETE "http://localhost:8000/cache?pattern=user:123:*"
# Response: {"deleted": 15, "status": "completed"}

# Delete by tag
curl -X DELETE "http://localhost:8000/cache?tag=user:123"
# Response: {"deleted": 8, "status": "completed"}

# Set up CDC invalidation
curl -X POST http://localhost:8000/admin/invalidation/cdc \
  -d '{
    "source": "kafka://localhost:9092/db-changes",
    "rules": [
      {"table": "users", "tag_template": "user:{{id}}"},
      {"table": "orders", "tag_template": "user:{{user_id}}"}
    ]
  }'

# Monitor invalidation propagation
curl http://localhost:8000/admin/invalidation/stats
# {
#   "invalidations_24h": 50000,
#   "avg_propagation_ms": 45,
#   "p99_propagation_ms": 95
# }
```

#### Test Requirements
- [ ] Test key invalidation
- [ ] Test pattern invalidation
- [ ] Test tag-based invalidation
- [ ] Test multi-instance propagation
- [ ] Test CDC integration
- [ ] Test propagation latency < 100ms
