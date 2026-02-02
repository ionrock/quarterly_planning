---
id: "test-015"
title: "Rate Limiter Library"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a rate limiter library supporting multiple algorithms (token bucket, sliding window, fixed window). Can be used for API rate limiting, both in-memory and with Redis backend. Written in Python.

## Constraints

- Thread-safe for concurrent access
- Sub-millisecond overhead per check

## Implementation Notes

### Technology Stack
- **Language:** Python 3.11+
- **Redis Client:** redis-py with async support
- **Async:** asyncio compatible
- **Time:** monotonic clock for accuracy

### Core Interface
```python
from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Optional

@dataclass
class RateLimitResult:
    allowed: bool
    remaining: int
    reset_at: float  # Unix timestamp
    retry_after: Optional[float] = None  # Seconds until next allowed request

class RateLimiter(ABC):
    @abstractmethod
    async def is_allowed(self, key: str, cost: int = 1) -> RateLimitResult:
        """Check if request is allowed and consume quota if so."""
        pass

    @abstractmethod
    async def get_remaining(self, key: str) -> int:
        """Get remaining quota without consuming."""
        pass

    @abstractmethod
    async def reset(self, key: str) -> None:
        """Reset quota for a key."""
        pass
```

### Token Bucket Implementation
```python
import time
import threading
from dataclasses import dataclass, field

@dataclass
class TokenBucket:
    """Thread-safe token bucket rate limiter."""
    capacity: int
    refill_rate: float  # tokens per second
    _buckets: dict = field(default_factory=dict)
    _lock: threading.Lock = field(default_factory=threading.Lock)

    async def is_allowed(self, key: str, cost: int = 1) -> RateLimitResult:
        now = time.monotonic()

        with self._lock:
            if key not in self._buckets:
                self._buckets[key] = {
                    'tokens': self.capacity,
                    'last_refill': now,
                }

            bucket = self._buckets[key]

            # Refill tokens based on elapsed time
            elapsed = now - bucket['last_refill']
            refill = elapsed * self.refill_rate
            bucket['tokens'] = min(self.capacity, bucket['tokens'] + refill)
            bucket['last_refill'] = now

            if bucket['tokens'] >= cost:
                bucket['tokens'] -= cost
                return RateLimitResult(
                    allowed=True,
                    remaining=int(bucket['tokens']),
                    reset_at=now + (self.capacity - bucket['tokens']) / self.refill_rate,
                )
            else:
                wait_time = (cost - bucket['tokens']) / self.refill_rate
                return RateLimitResult(
                    allowed=False,
                    remaining=0,
                    reset_at=now + wait_time,
                    retry_after=wait_time,
                )
```

### Sliding Window Log
```python
@dataclass
class SlidingWindowLog:
    """Precise sliding window using request timestamps."""
    limit: int
    window_seconds: float
    _windows: dict = field(default_factory=dict)
    _lock: threading.Lock = field(default_factory=threading.Lock)

    async def is_allowed(self, key: str, cost: int = 1) -> RateLimitResult:
        now = time.monotonic()
        window_start = now - self.window_seconds

        with self._lock:
            if key not in self._windows:
                self._windows[key] = []

            # Remove expired entries
            self._windows[key] = [
                ts for ts in self._windows[key] if ts > window_start
            ]

            current_count = len(self._windows[key])

            if current_count + cost <= self.limit:
                # Add timestamps for this request
                self._windows[key].extend([now] * cost)
                return RateLimitResult(
                    allowed=True,
                    remaining=self.limit - current_count - cost,
                    reset_at=now + self.window_seconds,
                )
            else:
                oldest = self._windows[key][0] if self._windows[key] else now
                retry_after = oldest + self.window_seconds - now
                return RateLimitResult(
                    allowed=False,
                    remaining=0,
                    reset_at=oldest + self.window_seconds,
                    retry_after=max(0, retry_after),
                )
```

### Redis Backend (Lua Script)
```python
class RedisRateLimiter:
    """Distributed rate limiter using Redis."""

    SLIDING_WINDOW_SCRIPT = """
    local key = KEYS[1]
    local limit = tonumber(ARGV[1])
    local window = tonumber(ARGV[2])
    local now = tonumber(ARGV[3])
    local cost = tonumber(ARGV[4])

    -- Remove old entries
    redis.call('ZREMRANGEBYSCORE', key, '-inf', now - window)

    -- Count current requests
    local count = redis.call('ZCARD', key)

    if count + cost <= limit then
        -- Add new request(s)
        for i = 1, cost do
            redis.call('ZADD', key, now, now .. ':' .. i .. ':' .. math.random())
        end
        redis.call('EXPIRE', key, window)
        return {1, limit - count - cost, now + window}
    else
        local oldest = redis.call('ZRANGE', key, 0, 0, 'WITHSCORES')
        local retry = oldest[2] and (oldest[2] + window - now) or window
        return {0, 0, oldest[2] and oldest[2] + window or now + window, retry}
    end
    """

    def __init__(self, redis_client, limit: int, window_seconds: float):
        self.redis = redis_client
        self.limit = limit
        self.window = window_seconds
        self._script = self.redis.register_script(self.SLIDING_WINDOW_SCRIPT)

    async def is_allowed(self, key: str, cost: int = 1) -> RateLimitResult:
        now = time.time()
        result = await self._script(
            keys=[f"ratelimit:{key}"],
            args=[self.limit, self.window, now, cost],
        )
        return RateLimitResult(
            allowed=bool(result[0]),
            remaining=int(result[1]),
            reset_at=float(result[2]),
            retry_after=float(result[3]) if len(result) > 3 else None,
        )
```

### FastAPI Middleware Example
```python
from fastapi import Request, HTTPException

async def rate_limit_middleware(request: Request, call_next):
    limiter = request.app.state.rate_limiter
    key = request.client.host  # Or use user ID, API key, etc.

    result = await limiter.is_allowed(key)
    if not result.allowed:
        raise HTTPException(
            status_code=429,
            detail="Rate limit exceeded",
            headers={
                "X-RateLimit-Remaining": str(result.remaining),
                "X-RateLimit-Reset": str(int(result.reset_at)),
                "Retry-After": str(int(result.retry_after)),
            },
        )

    response = await call_next(request)
    response.headers["X-RateLimit-Remaining"] = str(result.remaining)
    response.headers["X-RateLimit-Reset"] = str(int(result.reset_at))
    return response
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: Algorithm Implementations

**Summary:** Implement token bucket, sliding window, and fixed window algorithms.

**Definition of Done:** All algorithms correctly limit rates.

### Ticket 2: Redis Backend

**Summary:** Implement Redis-backed rate limiter for distributed use.

**Definition of Done:** Rate limits are shared across multiple instances.

### Ticket 3: API Design

**Summary:** Design clean API for checking and consuming quota.

**Definition of Done:** API is intuitive and well-documented.
