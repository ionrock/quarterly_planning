---
id: "test-022"
title: "Configuration Management System"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a centralized configuration management system for distributed applications. Supports versioning, environment-specific configs, and live updates.

## Constraints

- Config fetch latency under 10ms
- Support 10,000 config reads per second

## Implementation Notes

- Go backend
- etcd for storage
- gRPC for clients

## Review Notes

(none yet)

## Tickets

### Ticket 1: Config Storage

**Summary:** Store and version configuration data.

**Definition of Done:** Configs stored with full version history.

#### Acceptance Criteria

1. **Config Structure**
   - [ ] Hierarchical namespaces: /app/service/key
   - [ ] Support JSON, YAML, plain text values
   - [ ] Maximum value size: 1MB
   - [ ] Metadata: description, owner, updated_by

2. **Versioning**
   - [ ] Every change creates new version
   - [ ] Version number auto-incremented
   - [ ] Full history retained (configurable retention)
   - [ ] Diff between versions available

3. **CRUD Operations**
   - [ ] GET /config/{path} retrieves config
   - [ ] PUT /config/{path} creates/updates
   - [ ] DELETE /config/{path} removes config
   - [ ] LIST /config/{prefix} lists children

4. **Access Control**
   - [ ] Service accounts with API keys
   - [ ] Read/write permissions per namespace
   - [ ] Audit log of all changes
   - [ ] Approval workflow (optional)

5. **Validation**
   - [ ] JSON Schema validation per config
   - [ ] Type checking
   - [ ] Required fields enforcement
   - [ ] Custom validators (regex, enum)

#### Demo Script
```bash
# Set config value
curl -X PUT http://localhost:8000/api/config/myapp/database/connection_string \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "value": "postgres://localhost:5432/mydb",
    "description": "Primary database connection",
    "schema": {"type": "string", "format": "uri"}
  }'
# Response: {"path": "/myapp/database/connection_string", "version": 1}

# Get config
curl http://localhost:8000/api/config/myapp/database/connection_string
# Response: {"value": "postgres://...", "version": 1, "updated_at": "..."}

# Update config
curl -X PUT http://localhost:8000/api/config/myapp/database/connection_string \
  -d '{"value": "postgres://newhost:5432/mydb"}'
# Response: {"path": "...", "version": 2}

# Get version history
curl http://localhost:8000/api/config/myapp/database/connection_string/history
# Response: [{"version": 2, "value": "...", "updated_at": "..."}, {"version": 1, ...}]

# Get specific version
curl http://localhost:8000/api/config/myapp/database/connection_string?version=1

# List configs
curl http://localhost:8000/api/config/myapp/database
# Response: {"children": ["connection_string", "pool_size", "timeout"]}
```

#### Test Requirements
- [ ] Test CRUD operations
- [ ] Test versioning
- [ ] Test validation (schema)
- [ ] Test access control
- [ ] Test audit logging
- [ ] Test concurrent updates

### Ticket 2: Environment Management

**Summary:** Environment-specific config overrides.

**Definition of Done:** Configs resolve correctly per environment.

#### Acceptance Criteria

1. **Environment Definition**
   - [ ] Environments: dev, staging, prod (configurable)
   - [ ] Environment hierarchy for inheritance
   - [ ] Default values at base level
   - [ ] Override at environment level

2. **Resolution Logic**
   - [ ] Look up: /env/{environment}/{path}
   - [ ] Fall back to: /{path} if not found
   - [ ] Merge strategy for objects
   - [ ] Override strategy for primitives

3. **Environment Variables**
   - [ ] Reference env vars in config: ${DB_HOST}
   - [ ] Environment-specific var substitution
   - [ ] Default values: ${DB_HOST:localhost}

4. **Secret References**
   - [ ] Reference secrets: ${secret:db-password}
   - [ ] Secrets stored separately (Vault integration)
   - [ ] Never log or expose secret values

5. **Config Bundles**
   - [ ] Export all configs for environment
   - [ ] Generate .env file
   - [ ] Generate Kubernetes ConfigMap

#### Demo Script
```bash
# Set base config
curl -X PUT http://localhost:8000/api/config/myapp/database/host \
  -d '{"value": "localhost"}'

# Set production override
curl -X PUT http://localhost:8000/api/config/env/prod/myapp/database/host \
  -d '{"value": "prod-db.internal"}'

# Get config for dev (uses base)
curl "http://localhost:8000/api/config/myapp/database/host?env=dev"
# Response: {"value": "localhost"}

# Get config for prod (uses override)
curl "http://localhost:8000/api/config/myapp/database/host?env=prod"
# Response: {"value": "prod-db.internal"}

# Config with secret reference
curl -X PUT http://localhost:8000/api/config/myapp/database/password \
  -d '{"value": "${secret:db-password}"}'

# Resolve config (replaces secrets)
curl "http://localhost:8000/api/config/myapp/database?env=prod&resolve=true"
# Response: {"host": "prod-db.internal", "password": "actual-secret-value"}

# Export as ConfigMap
curl "http://localhost:8000/api/config/myapp?env=prod&format=k8s-configmap"
# Response: apiVersion: v1, kind: ConfigMap, data: {...}
```

#### Test Requirements
- [ ] Test environment inheritance
- [ ] Test override resolution
- [ ] Test secret reference substitution
- [ ] Test bundle export
- [ ] Test environment variable substitution
- [ ] Test with multiple environments

### Ticket 3: Live Updates

**Summary:** Push config changes to running applications.

**Definition of Done:** Apps receive updates within 1 second.

#### Acceptance Criteria

1. **Watch API**
   - [ ] gRPC streaming watch
   - [ ] HTTP SSE alternative
   - [ ] Watch specific path or prefix
   - [ ] Receive only changes (not full config)

2. **Change Notification**
   - [ ] Event: path, old value, new value, version
   - [ ] Batch multiple changes
   - [ ] Ordered delivery
   - [ ] At-least-once delivery

3. **Client SDK**
   - [ ] Go, Python, Node.js SDKs
   - [ ] Auto-reconnect on disconnect
   - [ ] Local caching
   - [ ] Callback on change

4. **Rollout Control**
   - [ ] Staged rollout (10%, 50%, 100%)
   - [ ] Canary release to specific instances
   - [ ] Instant rollback
   - [ ] Change impact preview

5. **Health Monitoring**
   - [ ] Track connected clients
   - [ ] Monitor update propagation time
   - [ ] Alert on disconnected clients
   - [ ] Dashboard with client status

#### Demo Script
```go
// Go SDK usage
import "github.com/company/config-client"

client := config.NewClient("localhost:8000", config.Options{
    Environment: "prod",
    CachePath:   "/tmp/config-cache",
})

// Get config with local cache
value := client.Get("/myapp/database/host")

// Watch for changes
client.Watch("/myapp/database", func(event config.Event) {
    log.Printf("Config changed: %s = %s (v%d)",
        event.Path, event.Value, event.Version)
    // Reload database connection pool
    reloadDatabase()
})

// In server logs:
# Config changed: /myapp/database/host = new-host.internal (v3)
```

```bash
# HTTP SSE watch
curl -N "http://localhost:8000/api/config/myapp?watch=true&env=prod"
# data: {"type":"update","path":"/myapp/database/host","value":"new-host"}
# data: {"type":"update","path":"/myapp/cache/ttl","value":"300"}

# View connected clients
curl http://localhost:8000/api/clients
# Response: [{"id": "app-1", "watching": ["/myapp"], "connected_at": "..."}]

# Staged rollout
curl -X POST http://localhost:8000/api/config/myapp/database/host/rollout \
  -d '{"value": "new-host.internal", "stages": [10, 50, 100], "interval": "5m"}'
# Response: {"rollout_id": "roll_123", "status": "in_progress", "stage": 1}

# Check rollout status
curl http://localhost:8000/api/rollouts/roll_123
# Response: {"stage": 2, "progress": 50, "healthy": true}

# Rollback
curl -X POST http://localhost:8000/api/rollouts/roll_123/rollback
```

#### Test Requirements
- [ ] Test watch subscription
- [ ] Test change notification delivery
- [ ] Test reconnection handling
- [ ] Test local caching
- [ ] Test propagation latency < 1 second
- [ ] Test staged rollout
- [ ] Test rollback
- [ ] Load test: 10,000 clients watching
