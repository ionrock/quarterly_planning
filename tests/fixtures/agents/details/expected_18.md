---
id: "test-018"
title: "Database Connection Pool"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Implement a database connection pool for PostgreSQL in Go. Manages a pool of reusable connections, handles connection health checks, and supports graceful shutdown. Improves performance over creating connections per request.

## Constraints

- Support at least 100 concurrent connections
- Connection acquisition under 1ms when pool has available connections

## Implementation Notes

### Technology Stack
- **Language:** Go 1.21+
- **Driver:** pgx v5 (low-level) or lib/pq
- **Sync:** sync.Pool, sync.Cond for coordination

### Core Data Structures
```go
type Pool struct {
    config     PoolConfig
    conns      chan *Conn
    mu         sync.Mutex
    cond       *sync.Cond
    active     int
    closed     bool
    healthTick *time.Ticker

    // Metrics
    acquireCount  atomic.Int64
    releaseCount  atomic.Int64
    timeoutCount  atomic.Int64
    healthyConns  atomic.Int32
}

type PoolConfig struct {
    DSN             string
    MinConns        int           // Minimum idle connections
    MaxConns        int           // Maximum total connections
    AcquireTimeout  time.Duration // Max wait for connection
    IdleTimeout     time.Duration // Close idle connections after
    MaxLifetime     time.Duration // Max connection age
    HealthCheckInterval time.Duration
}

type Conn struct {
    raw       *pgx.Conn
    pool      *Pool
    createdAt time.Time
    lastUsed  time.Time
    inUse     bool
}
```

### Pool Initialization
```go
func NewPool(ctx context.Context, config PoolConfig) (*Pool, error) {
    if config.MinConns <= 0 {
        config.MinConns = 2
    }
    if config.MaxConns <= 0 {
        config.MaxConns = 10
    }
    if config.AcquireTimeout <= 0 {
        config.AcquireTimeout = 30 * time.Second
    }

    p := &Pool{
        config: config,
        conns:  make(chan *Conn, config.MaxConns),
    }
    p.cond = sync.NewCond(&p.mu)

    // Create minimum connections
    for i := 0; i < config.MinConns; i++ {
        conn, err := p.newConn(ctx)
        if err != nil {
            p.Close()
            return nil, fmt.Errorf("create initial connection: %w", err)
        }
        p.conns <- conn
    }

    // Start health checker
    p.healthTick = time.NewTicker(config.HealthCheckInterval)
    go p.healthCheckLoop()

    return p, nil
}

func (p *Pool) newConn(ctx context.Context) (*Conn, error) {
    raw, err := pgx.Connect(ctx, p.config.DSN)
    if err != nil {
        return nil, err
    }
    return &Conn{
        raw:       raw,
        pool:      p,
        createdAt: time.Now(),
        lastUsed:  time.Now(),
    }, nil
}
```

### Acquire Connection
```go
func (p *Pool) Acquire(ctx context.Context) (*Conn, error) {
    p.acquireCount.Add(1)

    // Fast path: try to get from channel without blocking
    select {
    case conn := <-p.conns:
        if p.isHealthy(conn) {
            conn.inUse = true
            conn.lastUsed = time.Now()
            return conn, nil
        }
        // Unhealthy, close and try again
        conn.raw.Close(ctx)
        p.mu.Lock()
        p.active--
        p.mu.Unlock()
    default:
    }

    // Try to create new connection if under limit
    p.mu.Lock()
    if p.active < p.config.MaxConns && !p.closed {
        p.active++
        p.mu.Unlock()

        conn, err := p.newConn(ctx)
        if err != nil {
            p.mu.Lock()
            p.active--
            p.mu.Unlock()
            return nil, err
        }
        conn.inUse = true
        return conn, nil
    }
    p.mu.Unlock()

    // Wait for available connection with timeout
    deadline := time.Now().Add(p.config.AcquireTimeout)
    for {
        select {
        case <-ctx.Done():
            return nil, ctx.Err()
        case conn := <-p.conns:
            if p.isHealthy(conn) {
                conn.inUse = true
                conn.lastUsed = time.Now()
                return conn, nil
            }
            conn.raw.Close(ctx)
        case <-time.After(time.Until(deadline)):
            p.timeoutCount.Add(1)
            return nil, ErrAcquireTimeout
        }
    }
}
```

### Release Connection
```go
func (p *Pool) Release(conn *Conn) {
    p.releaseCount.Add(1)

    if !conn.inUse {
        panic("releasing connection not in use")
    }
    conn.inUse = false
    conn.lastUsed = time.Now()

    p.mu.Lock()
    if p.closed {
        p.mu.Unlock()
        conn.raw.Close(context.Background())
        return
    }
    p.mu.Unlock()

    // Check if connection should be retired
    if time.Since(conn.createdAt) > p.config.MaxLifetime {
        conn.raw.Close(context.Background())
        p.mu.Lock()
        p.active--
        p.cond.Signal()
        p.mu.Unlock()
        return
    }

    // Return to pool
    select {
    case p.conns <- conn:
    default:
        // Pool full, close connection
        conn.raw.Close(context.Background())
        p.mu.Lock()
        p.active--
        p.mu.Unlock()
    }
}
```

### Health Check
```go
func (p *Pool) healthCheckLoop() {
    for range p.healthTick.C {
        p.mu.Lock()
        if p.closed {
            p.mu.Unlock()
            return
        }
        p.mu.Unlock()

        p.runHealthCheck()
    }
}

func (p *Pool) runHealthCheck() {
    ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
    defer cancel()

    // Check each connection in pool
    healthy := 0
    checked := 0
    maxToCheck := len(p.conns)

    for checked < maxToCheck {
        select {
        case conn := <-p.conns:
            checked++
            if p.isHealthy(conn) {
                healthy++
                p.conns <- conn
            } else {
                conn.raw.Close(ctx)
                p.mu.Lock()
                p.active--
                p.mu.Unlock()
            }
        default:
            break
        }
    }

    p.healthyConns.Store(int32(healthy))
}

func (p *Pool) isHealthy(conn *Conn) bool {
    // Check idle timeout
    if p.config.IdleTimeout > 0 && time.Since(conn.lastUsed) > p.config.IdleTimeout {
        return false
    }
    // Ping
    ctx, cancel := context.WithTimeout(context.Background(), time.Second)
    defer cancel()
    return conn.raw.Ping(ctx) == nil
}
```

### Graceful Shutdown
```go
func (p *Pool) Close() error {
    p.mu.Lock()
    if p.closed {
        p.mu.Unlock()
        return nil
    }
    p.closed = true
    p.mu.Unlock()

    p.healthTick.Stop()

    // Close all connections
    close(p.conns)
    for conn := range p.conns {
        conn.raw.Close(context.Background())
    }

    return nil
}
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: Pool Management

**Summary:** Implement connection pool with acquire/release operations.

**Definition of Done:** Connections are reused correctly.

### Ticket 2: Health Checks

**Summary:** Implement background health checking for pooled connections.

**Definition of Done:** Unhealthy connections are removed and replaced.

### Ticket 3: Graceful Shutdown

**Summary:** Implement shutdown that waits for in-use connections.

**Definition of Done:** Shutdown completes cleanly without dropped connections.
