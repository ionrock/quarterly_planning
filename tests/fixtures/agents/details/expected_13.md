---
id: "test-013"
title: "Feature Flag Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a feature flag service that allows toggling features without deployments. Supports percentage rollouts, user targeting, and A/B testing. Provides SDKs for server-side evaluation. Built with Go and PostgreSQL.

## Constraints

- Evaluation latency under 5ms
- Support 100,000+ evaluations per second

## Implementation Notes

### Technology Stack
- **Language:** Go 1.21+
- **Database:** PostgreSQL for flag storage
- **Cache:** In-memory with sync.Map for fast evaluation
- **API:** gRPC for SDK communication, REST for management

### Database Schema
```sql
CREATE TABLE flags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    enabled BOOLEAN NOT NULL DEFAULT false,
    default_value JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE flag_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    flag_id UUID NOT NULL REFERENCES flags(id) ON DELETE CASCADE,
    priority INT NOT NULL,
    conditions JSONB NOT NULL,  -- [{"attribute": "country", "op": "eq", "value": "US"}]
    value JSONB NOT NULL,
    percentage INT,  -- NULL = 100%, 0-100 for rollouts
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE flag_overrides (
    flag_id UUID NOT NULL REFERENCES flags(id) ON DELETE CASCADE,
    user_id VARCHAR(255) NOT NULL,
    value JSONB NOT NULL,
    PRIMARY KEY (flag_id, user_id)
);

CREATE INDEX idx_flag_rules_flag ON flag_rules(flag_id, priority);
```

### Core Data Structures
```go
type Flag struct {
    ID           string          `json:"id"`
    Key          string          `json:"key"`
    Enabled      bool            `json:"enabled"`
    DefaultValue json.RawMessage `json:"defaultValue"`
    Rules        []Rule          `json:"rules"`
}

type Rule struct {
    ID         string      `json:"id"`
    Priority   int         `json:"priority"`
    Conditions []Condition `json:"conditions"`
    Value      json.RawMessage `json:"value"`
    Percentage *int        `json:"percentage,omitempty"`
}

type Condition struct {
    Attribute string `json:"attribute"`
    Operator  string `json:"op"`  // eq, neq, in, contains, gt, lt, regex
    Value     any    `json:"value"`
}

type EvaluationContext struct {
    UserID     string            `json:"userId"`
    Attributes map[string]any    `json:"attributes"`
}
```

### Evaluation Algorithm
```go
func (e *Evaluator) Evaluate(flagKey string, ctx EvaluationContext) (json.RawMessage, error) {
    flag, ok := e.cache.Load(flagKey)
    if !ok {
        return nil, ErrFlagNotFound
    }

    if !flag.Enabled {
        return flag.DefaultValue, nil
    }

    // Check user override first
    if override, ok := e.getOverride(flagKey, ctx.UserID); ok {
        return override, nil
    }

    // Evaluate rules in priority order
    for _, rule := range flag.Rules {
        if e.matchesConditions(rule.Conditions, ctx) {
            if rule.Percentage != nil {
                // Consistent hashing for percentage rollout
                if !e.isInPercentage(flagKey, ctx.UserID, *rule.Percentage) {
                    continue
                }
            }
            return rule.Value, nil
        }
    }

    return flag.DefaultValue, nil
}

// Consistent hashing ensures same user always gets same result
func (e *Evaluator) isInPercentage(flagKey, userID string, percentage int) bool {
    hash := murmur3.Sum32([]byte(flagKey + ":" + userID))
    bucket := hash % 100
    return int(bucket) < percentage
}
```

### Condition Matching
```go
func (e *Evaluator) matchesCondition(cond Condition, ctx EvaluationContext) bool {
    attrValue, ok := ctx.Attributes[cond.Attribute]
    if !ok {
        return false
    }

    switch cond.Operator {
    case "eq":
        return reflect.DeepEqual(attrValue, cond.Value)
    case "neq":
        return !reflect.DeepEqual(attrValue, cond.Value)
    case "in":
        list, ok := cond.Value.([]any)
        if !ok { return false }
        return slices.Contains(list, attrValue)
    case "contains":
        str, ok := attrValue.(string)
        if !ok { return false }
        return strings.Contains(str, cond.Value.(string))
    case "gt", "lt", "gte", "lte":
        return compareNumbers(attrValue, cond.Value, cond.Operator)
    case "regex":
        str, ok := attrValue.(string)
        if !ok { return false }
        re, _ := regexp.Compile(cond.Value.(string))
        return re.MatchString(str)
    }
    return false
}
```

### SDK Client (Go)
```go
type Client struct {
    evaluator *Evaluator
    conn      *grpc.ClientConn
}

func NewClient(addr string, opts ...Option) (*Client, error) {
    conn, err := grpc.Dial(addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
    if err != nil {
        return nil, err
    }
    client := &Client{conn: conn, evaluator: NewEvaluator()}

    // Start background sync
    go client.syncLoop(context.Background())

    return client, nil
}

func (c *Client) BoolValue(flagKey string, ctx EvaluationContext, defaultValue bool) bool {
    result, err := c.evaluator.Evaluate(flagKey, ctx)
    if err != nil {
        return defaultValue
    }
    var v bool
    json.Unmarshal(result, &v)
    return v
}
```

### Cache Sync (Polling)
```go
func (c *Client) syncLoop(ctx context.Context) {
    ticker := time.NewTicker(30 * time.Second)
    defer ticker.Stop()

    for {
        select {
        case <-ctx.Done():
            return
        case <-ticker.C:
            c.sync(ctx)
        }
    }
}
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: Flag Management API

**Summary:** CRUD API for creating and updating feature flags.

**Definition of Done:** Flags can be created, updated, and deleted via API.

### Ticket 2: Evaluation Engine

**Summary:** Implement flag evaluation with targeting rules.

**Definition of Done:** Flags evaluate correctly based on user attributes.

### Ticket 3: SDK Implementation

**Summary:** Create Go SDK for server-side flag evaluation.

**Definition of Done:** SDK returns flag values with low latency.
