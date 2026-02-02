---
id: "test-024"
title: "Distributed Lock Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Implement a distributed lock service for coordinating access to shared resources across multiple services. Provides mutual exclusion with automatic lock expiration. Built on Redis with a Go client library.

## Constraints

- Lock acquisition under 10ms
- Must handle Redis failover

## Implementation Notes

### Technology Stack
- **Language:** Go 1.21+
- **Redis Client:** go-redis/redis v9
- **Algorithm:** Redlock for multi-node safety

### Lock Interface
```go
type Lock interface {
    // Acquire attempts to acquire the lock with timeout
    Acquire(ctx context.Context) error

    // Release releases the lock if still held
    Release(ctx context.Context) error

    // Extend extends the lock TTL
    Extend(ctx context.Context, ttl time.Duration) error

    // IsHeld returns true if lock is still held by this instance
    IsHeld() bool
}

type LockOptions struct {
    Key        string
    TTL        time.Duration // Lock expiration
    RetryDelay time.Duration // Delay between retries
    RetryCount int           // Max retry attempts
}
```

### Single-Node Lock Implementation
```go
type redisLock struct {
    client *redis.Client
    key    string
    value  string // Unique token for this lock holder
    ttl    time.Duration
    mu     sync.Mutex
}

const lockScript = `
if redis.call("get", KEYS[1]) == ARGV[1] then
    return redis.call("del", KEYS[1])
else
    return 0
end
`

const extendScript = `
if redis.call("get", KEYS[1]) == ARGV[1] then
    return redis.call("pexpire", KEYS[1], ARGV[2])
else
    return 0
end
`

func NewLock(client *redis.Client, opts LockOptions) *redisLock {
    return &redisLock{
        client: client,
        key:    "lock:" + opts.Key,
        value:  uuid.New().String(),
        ttl:    opts.TTL,
    }
}

func (l *redisLock) Acquire(ctx context.Context) error {
    l.mu.Lock()
    defer l.mu.Unlock()

    ok, err := l.client.SetNX(ctx, l.key, l.value, l.ttl).Result()
    if err != nil {
        return fmt.Errorf("acquire lock: %w", err)
    }
    if !ok {
        return ErrLockNotAcquired
    }
    return nil
}

func (l *redisLock) Release(ctx context.Context) error {
    l.mu.Lock()
    defer l.mu.Unlock()

    result, err := l.client.Eval(ctx, lockScript, []string{l.key}, l.value).Int()
    if err != nil {
        return fmt.Errorf("release lock: %w", err)
    }
    if result == 0 {
        return ErrLockNotHeld
    }
    return nil
}

func (l *redisLock) Extend(ctx context.Context, ttl time.Duration) error {
    l.mu.Lock()
    defer l.mu.Unlock()

    result, err := l.client.Eval(ctx, extendScript, []string{l.key}, l.value, ttl.Milliseconds()).Int()
    if err != nil {
        return fmt.Errorf("extend lock: %w", err)
    }
    if result == 0 {
        return ErrLockNotHeld
    }
    return nil
}
```

### Redlock Algorithm (Multi-Node)
```go
type Redlock struct {
    clients  []*redis.Client
    quorum   int
    opts     LockOptions
}

func NewRedlock(clients []*redis.Client, opts LockOptions) *Redlock {
    return &Redlock{
        clients: clients,
        quorum:  len(clients)/2 + 1,
        opts:    opts,
    }
}

func (r *Redlock) Acquire(ctx context.Context) (*redlockLock, error) {
    value := uuid.New().String()
    startTime := time.Now()
    drift := time.Duration(float64(r.opts.TTL) * 0.01) // Clock drift factor

    for attempt := 0; attempt <= r.opts.RetryCount; attempt++ {
        if attempt > 0 {
            time.Sleep(r.opts.RetryDelay)
        }

        acquired := 0
        var wg sync.WaitGroup
        var mu sync.Mutex

        for _, client := range r.clients {
            wg.Add(1)
            go func(c *redis.Client) {
                defer wg.Done()
                ok, err := c.SetNX(ctx, r.opts.Key, value, r.opts.TTL).Result()
                if err == nil && ok {
                    mu.Lock()
                    acquired++
                    mu.Unlock()
                }
            }(client)
        }
        wg.Wait()

        // Check if we have quorum and enough time left
        elapsed := time.Since(startTime)
        validity := r.opts.TTL - elapsed - drift

        if acquired >= r.quorum && validity > 0 {
            return &redlockLock{
                redlock:  r,
                value:    value,
                validity: validity,
            }, nil
        }

        // Failed to acquire, release any locks we got
        r.releaseAll(ctx, value)
    }

    return nil, ErrLockNotAcquired
}

func (r *Redlock) releaseAll(ctx context.Context, value string) {
    var wg sync.WaitGroup
    for _, client := range r.clients {
        wg.Add(1)
        go func(c *redis.Client) {
            defer wg.Done()
            c.Eval(ctx, lockScript, []string{r.opts.Key}, value)
        }(client)
    }
    wg.Wait()
}
```

### Auto-Renewal
```go
type AutoRenewLock struct {
    lock     Lock
    cancel   context.CancelFunc
    done     chan struct{}
    interval time.Duration
}

func WithAutoRenew(lock Lock, ttl time.Duration) *AutoRenewLock {
    ctx, cancel := context.WithCancel(context.Background())
    arl := &AutoRenewLock{
        lock:     lock,
        cancel:   cancel,
        done:     make(chan struct{}),
        interval: ttl / 3, // Renew at 1/3 of TTL
    }

    go arl.renewLoop(ctx, ttl)
    return arl
}

func (a *AutoRenewLock) renewLoop(ctx context.Context, ttl time.Duration) {
    ticker := time.NewTicker(a.interval)
    defer ticker.Stop()
    defer close(a.done)

    for {
        select {
        case <-ctx.Done():
            return
        case <-ticker.C:
            if err := a.lock.Extend(ctx, ttl); err != nil {
                log.Printf("failed to extend lock: %v", err)
                return
            }
        }
    }
}

func (a *AutoRenewLock) Release(ctx context.Context) error {
    a.cancel()
    <-a.done
    return a.lock.Release(ctx)
}
```

### Client Library Usage
```go
func Example() {
    client := redis.NewClient(&redis.Options{Addr: "localhost:6379"})

    lock := lock.NewLock(client, lock.LockOptions{
        Key:        "my-resource",
        TTL:        30 * time.Second,
        RetryDelay: 100 * time.Millisecond,
        RetryCount: 3,
    })

    ctx := context.Background()
    if err := lock.Acquire(ctx); err != nil {
        log.Fatal("failed to acquire lock:", err)
    }
    defer lock.Release(ctx)

    // Do work while holding lock
    doWork()
}
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: Lock Acquisition

**Summary:** Implement acquire with SET NX and expiration.

**Definition of Done:** Locks are acquired atomically.

### Ticket 2: Lock Extension

**Summary:** Allow extending lock TTL for long operations.

**Definition of Done:** Locks can be extended without release.

### Ticket 3: Client Library

**Summary:** Package as reusable Go library.

**Definition of Done:** Library is documented and easy to use.
