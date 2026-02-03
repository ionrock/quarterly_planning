---
id: "test-013"
title: "Feature Flag Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a feature flag service for controlling feature rollouts. Supports percentage rollouts, user targeting, and A/B testing. Includes SDK for multiple languages.

## Constraints

- Flag evaluation under 5ms
- Support 1 million daily active users

## Implementation Notes

- Go backend with REST API
- Redis for flag storage
- SSE for real-time updates

## Review Notes

(none yet)

## Tickets

### Ticket 1: Flag Management

**Summary:** CRUD operations for feature flags.

**Definition of Done:** Flags can be created and configured.

#### Steps

1. **Create Go project structure**
   - Run `go mod init github.com/example/flagservice`
   - Create cmd/server/main.go
   - Verify: `go build` succeeds

2. **Define Flag struct**
   - Create internal/models/flag.go
   - Include: Key, Name, Description, Enabled, Rules, CreatedAt, UpdatedAt
   - Verify: struct compiles

3. **Define Rule struct**
   - Create internal/models/rule.go
   - Include: Attribute, Operator, Value, Percentage, Variant
   - Verify: struct compiles

4. **Set up Redis connection**
   - Add go-redis/redis dependency
   - Create internal/store/redis.go
   - Verify: connects to Redis

5. **Implement flag storage**
   - Create internal/store/flag_store.go
   - Store flags as JSON in Redis hash
   - Verify: flags persist correctly

6. **Create HTTP router**
   - Use chi or gin framework
   - Create internal/api/router.go
   - Verify: server starts

7. **Implement POST /flags endpoint**
   - Accept flag JSON in body
   - Validate required fields
   - Store in Redis
   - Verify: flag created

8. **Implement GET /flags endpoint**
   - List all flags
   - Support pagination
   - Verify: flags listed

9. **Implement GET /flags/:key endpoint**
   - Fetch single flag by key
   - Return 404 if not found
   - Verify: flag retrieved

10. **Implement PUT /flags/:key endpoint**
    - Update flag configuration
    - Update timestamp
    - Verify: flag updated

11. **Implement DELETE /flags/:key endpoint**
    - Remove flag from storage
    - Verify: flag deleted

12. **Add audit logging**
    - Log all flag changes with user
    - Store in separate Redis stream
    - Verify: changes logged

### Ticket 2: Evaluation Engine

**Summary:** Evaluate flags based on targeting rules.

**Definition of Done:** Flags evaluate correctly for all rule types.

#### Steps

1. **Define EvaluationContext struct**
   - Create internal/eval/context.go
   - Include: UserID, Attributes (map[string]interface{})
   - Verify: struct compiles

2. **Define EvaluationResult struct**
   - Include: Enabled bool, Variant string, Reason string
   - Verify: struct compiles

3. **Create evaluator interface**
   - Create internal/eval/evaluator.go
   - Define Evaluate(flag, context) -> Result
   - Verify: interface compiles

4. **Implement enabled check**
   - Return disabled if flag.Enabled = false
   - Verify: disabled flags return false

5. **Implement percentage rollout**
   - Hash user ID to get consistent bucket
   - Check if bucket < percentage
   - Verify: ~50% get feature with 50% rollout

6. **Implement attribute matching**
   - Support operators: equals, notEquals, contains, startsWith, endsWith
   - Match against context attributes
   - Verify: attribute rules work

7. **Implement numeric comparisons**
   - Support: greaterThan, lessThan, greaterOrEqual, lessOrEqual
   - Verify: numeric rules work

8. **Implement list operations**
   - Support: in, notIn for attribute values
   - Verify: list rules work

9. **Implement rule combining**
   - Evaluate rules in order
   - First matching rule wins
   - Default to flag-level enabled state
   - Verify: rule priority works

10. **Implement variant selection**
    - Support multiple variants with weights
    - Assign variant based on hash bucket
    - Verify: variants distributed correctly

11. **Cache evaluation results**
    - Cache per user+flag for 1 minute
    - Verify: cache improves performance

12. **Create POST /evaluate endpoint**
    - Accept flagKey and context
    - Return evaluation result
    - Verify: endpoint works

### Ticket 3: Client SDK

**Summary:** Create SDK for flag consumption.

**Definition of Done:** SDK available for JavaScript and Python.

#### Steps

1. **Create JavaScript SDK package**
   - Create sdk/javascript directory
   - Initialize npm package
   - Verify: package structure correct

2. **Implement JS client initialization**
   - Accept apiUrl and apiKey
   - Verify: client initializes

3. **Implement JS evaluate function**
   - Call /evaluate endpoint
   - Pass context to server
   - Verify: evaluation works

4. **Add JS local caching**
   - Cache flag values in memory
   - Refresh on interval
   - Verify: reduces API calls

5. **Implement JS SSE connection**
   - Subscribe to flag changes
   - Update local cache on change
   - Verify: real-time updates work

6. **Create Python SDK package**
   - Create sdk/python directory
   - Create setup.py
   - Verify: package installable

7. **Implement Python client initialization**
   - Accept api_url and api_key
   - Verify: client initializes

8. **Implement Python evaluate function**
   - Call /evaluate endpoint
   - Return EvaluationResult
   - Verify: evaluation works

9. **Add Python local caching**
   - Cache flags in memory
   - Thread-safe access
   - Verify: concurrent access safe

10. **Implement Python SSE connection**
    - Use sseclient library
    - Update cache on events
    - Verify: real-time updates work

11. **Add offline mode to SDKs**
    - Load flags from file
    - Work without server connection
    - Verify: offline mode works

12. **Write SDK documentation**
    - Document initialization
    - Document evaluate API
    - Add examples
    - Verify: docs are clear

13. **Publish SDK packages**
    - Publish JS to npm
    - Publish Python to PyPI
    - Verify: packages installable
