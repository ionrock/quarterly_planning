---
id: "test-007"
title: "Feature Flag Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a feature flag service for gradual rollouts. Supports percentage-based rollouts, user targeting, and A/B testing. Provides SDKs for multiple languages.

## Constraints

- Flag evaluation under 5ms (p99)
- 99.99% availability

## Implementation Notes

- Go backend service
- PostgreSQL for flag storage
- Local caching in SDKs

## Review Notes

(none yet)

## Tickets

### Ticket 1: Flag Evaluation Engine

**Summary:** Implement flag evaluation with targeting rules.

**Definition of Done:** Flags evaluate correctly for all rule types.

#### Acceptance Criteria

1. **Boolean Flags**
   - [ ] Simple on/off flag returns true/false
   - [ ] Default value returned if flag doesn't exist
   - [ ] Disabled flag returns default value

2. **Percentage Rollout**
   - [ ] Flag enabled for X% of users
   - [ ] Consistent hashing (same user always gets same result)
   - [ ] Distribution within 1% of target percentage
   - [ ] Percentage can be 0-100 with decimal precision

3. **User Targeting**
   - [ ] Target by user ID (allowlist)
   - [ ] Target by user attribute (email domain, country, etc.)
   - [ ] Multiple rules combined with AND/OR
   - [ ] Rule operators: equals, contains, startsWith, regex, in, gt, lt

4. **Variants (A/B Testing)**
   - [ ] Flag can return multiple variants (not just true/false)
   - [ ] Variant weights configurable (e.g., 50/30/20)
   - [ ] User consistently assigned to same variant
   - [ ] Control group tracking

5. **Evaluation Context**
   - [ ] User ID (required for percentage/targeting)
   - [ ] Custom attributes (key-value pairs)
   - [ ] Anonymous users supported (device ID)

#### Demo Script
```go
// Initialize evaluator
evaluator := NewEvaluator(flagStore)

// Simple boolean
enabled := evaluator.IsEnabled("new-checkout", user)
// Returns: true or false

// With context
ctx := EvalContext{
    UserID: "user-123",
    Attributes: map[string]interface{}{
        "email": "user@company.com",
        "country": "US",
        "plan": "premium",
    },
}
enabled := evaluator.IsEnabled("beta-feature", ctx)

// Variants
variant := evaluator.GetVariant("checkout-flow", ctx)
// Returns: "control", "variant-a", or "variant-b"

// Default value
enabled := evaluator.IsEnabled("nonexistent-flag", ctx)
// Returns: false (default)
```

#### Test Requirements
- [ ] Unit tests for each rule type
- [ ] Test percentage distribution (10,000 users)
- [ ] Test consistent hashing
- [ ] Benchmark: 1M evaluations under 5s total
- [ ] Fuzz test with random rule combinations

### Ticket 2: Admin API

**Summary:** CRUD API for managing flags.

**Definition of Done:** Flags can be created, updated, and deleted.

#### Acceptance Criteria

1. **Create Flag**
   - [ ] POST /api/flags creates new flag
   - [ ] Required: key (unique), name, description
   - [ ] Optional: targeting rules, percentage, variants
   - [ ] Returns 201 with created flag
   - [ ] Returns 409 if key already exists

2. **Read Flags**
   - [ ] GET /api/flags lists all flags
   - [ ] Pagination: limit, offset parameters
   - [ ] Filter by: enabled, tag, search query
   - [ ] GET /api/flags/{key} returns single flag
   - [ ] Returns 404 if flag doesn't exist

3. **Update Flag**
   - [ ] PUT /api/flags/{key} updates entire flag
   - [ ] PATCH /api/flags/{key} partial update
   - [ ] Optimistic locking with version field
   - [ ] Returns 409 on version conflict

4. **Delete Flag**
   - [ ] DELETE /api/flags/{key} removes flag
   - [ ] Soft delete (archived) by default
   - [ ] Hard delete with ?permanent=true
   - [ ] Returns 204 on success

5. **Audit Log**
   - [ ] All changes logged with timestamp, user, action
   - [ ] GET /api/flags/{key}/history returns change history
   - [ ] Diff between versions viewable

#### Demo Script
```bash
# Create flag
curl -X POST http://localhost:8080/api/flags \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "key": "new-checkout",
    "name": "New Checkout Flow",
    "description": "Redesigned checkout experience",
    "enabled": false,
    "percentage": 0
  }'
# Response: 201, {"key": "new-checkout", "version": 1, ...}

# Enable for 10%
curl -X PATCH http://localhost:8080/api/flags/new-checkout \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"enabled": true, "percentage": 10, "version": 1}'
# Response: 200, {"key": "new-checkout", "version": 2, ...}

# Add targeting rule
curl -X PATCH http://localhost:8080/api/flags/new-checkout \
  -H "Content-Type: application/json" \
  -d '{
    "rules": [
      {"attribute": "email", "operator": "endsWith", "value": "@company.com"}
    ],
    "version": 2
  }'

# List flags
curl http://localhost:8080/api/flags?enabled=true
# Response: {"flags": [...], "total": 42, "limit": 20, "offset": 0}

# View history
curl http://localhost:8080/api/flags/new-checkout/history
# Response: [{"version": 2, "user": "admin", "changes": {...}, "timestamp": "..."}]
```

#### Test Requirements
- [ ] API integration tests
- [ ] Test CRUD operations
- [ ] Test optimistic locking
- [ ] Test pagination and filtering
- [ ] Test audit log completeness
- [ ] Load test: 100 concurrent flag updates

### Ticket 3: Client SDK (JavaScript)

**Summary:** JavaScript SDK for flag evaluation.

**Definition of Done:** SDK evaluates flags with local caching.

#### Acceptance Criteria

1. **Initialization**
   - [ ] Initialize with API key and endpoint
   - [ ] Fetch all flags on init
   - [ ] User context set on init or per-evaluation
   - [ ] Callback/promise when ready

2. **Flag Evaluation**
   - [ ] isEnabled(key): boolean
   - [ ] getVariant(key): string
   - [ ] getValue(key, default): any
   - [ ] Evaluation uses local cache (no network call)

3. **Caching**
   - [ ] Flags cached in memory
   - [ ] Background refresh every 30s (configurable)
   - [ ] Manual refresh available
   - [ ] Stale cache used if refresh fails

4. **Streaming Updates**
   - [ ] SSE connection for real-time updates
   - [ ] Automatic reconnection on disconnect
   - [ ] Callback on flag change
   - [ ] Falls back to polling if SSE unavailable

5. **Offline Support**
   - [ ] LocalStorage persistence
   - [ ] Works offline with cached flags
   - [ ] Bootstrap flags from config

6. **Events**
   - [ ] Track flag evaluations
   - [ ] Batch events and send periodically
   - [ ] Include evaluation result and context

#### Demo Script
```javascript
import { FeatureFlags } from '@company/feature-flags';

// Initialize
const flags = new FeatureFlags({
  apiKey: 'sdk-key-xxx',
  endpoint: 'https://flags.example.com',
  user: {
    id: 'user-123',
    email: 'user@example.com',
    attributes: { plan: 'premium' }
  }
});

await flags.ready();

// Evaluate flags
if (flags.isEnabled('new-checkout')) {
  showNewCheckout();
} else {
  showOldCheckout();
}

// Get variant
const variant = flags.getVariant('pricing-page');
// Returns: 'control' | 'variant-a' | 'variant-b'

// Listen for changes
flags.on('change', (key, newValue) => {
  console.log(`Flag ${key} changed to ${newValue}`);
  // Re-render affected components
});

// Update user context
flags.setUser({ id: 'user-456', email: 'other@example.com' });

// Manual refresh
await flags.refresh();
```

#### Test Requirements
- [ ] Unit tests for evaluation logic
- [ ] Test caching behavior
- [ ] Test offline mode
- [ ] Test SSE reconnection
- [ ] Test in browser environment (jest-dom)
- [ ] Test bundle size < 10KB gzipped
