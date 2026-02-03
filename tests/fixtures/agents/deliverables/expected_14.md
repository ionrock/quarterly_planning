---
id: "test-014"
title: "API Mocking Server"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a mock API server that simulates backend services for frontend development and testing. Supports OpenAPI spec import, dynamic responses, and request recording.

## Constraints

- Response latency under 10ms
- Support 1000 concurrent connections

## Implementation Notes

- Node.js with Fastify
- JSON Schema validation
- WebSocket support

## Review Notes

(none yet)

## Tickets

### Ticket 1: Mock Definition

**Summary:** Define mock endpoints with response templates.

**Definition of Done:** Mocks configurable via YAML/JSON.

#### Acceptance Criteria

1. **Mock Configuration**
   - [ ] YAML or JSON configuration files
   - [ ] Define: method, path, response status, body, headers
   - [ ] Path parameters: /users/:id
   - [ ] Query parameter matching

2. **OpenAPI Import**
   - [ ] Import from OpenAPI 3.0 spec
   - [ ] Generate mock responses from examples
   - [ ] Generate from schema if no examples
   - [ ] Validate requests against spec

3. **Response Definition**
   - [ ] Static JSON response
   - [ ] Response varies by request parameter
   - [ ] Multiple responses with conditions
   - [ ] Custom headers per response

4. **Hot Reload**
   - [ ] Watch config files for changes
   - [ ] Reload without restart
   - [ ] Validation errors shown in console
   - [ ] Keep existing connections alive

5. **Endpoint Management**
   - [ ] List all mocked endpoints
   - [ ] Add/update/delete via admin API
   - [ ] Import/export configurations

#### Demo Script
```yaml
# mocks/users.yaml
endpoints:
  - method: GET
    path: /api/users
    response:
      status: 200
      body:
        - id: 1
          name: "Alice"
        - id: 2
          name: "Bob"

  - method: GET
    path: /api/users/:id
    responses:
      - when:
          params:
            id: "1"
        status: 200
        body: { id: 1, name: "Alice", email: "alice@example.com" }
      - when:
          params:
            id: "999"
        status: 404
        body: { error: "User not found" }
      - default: true
        status: 200
        body: { id: "{{params.id}}", name: "User {{params.id}}" }

  - method: POST
    path: /api/users
    response:
      status: 201
      body: { id: "{{faker.uuid}}", name: "{{request.body.name}}" }
```

```bash
# Start mock server
mock-server start --config mocks/ --port 3000

# Test endpoints
curl http://localhost:3000/api/users
# [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]

curl http://localhost:3000/api/users/1
# {"id": 1, "name": "Alice", "email": "alice@example.com"}

curl http://localhost:3000/api/users/999
# {"error": "User not found"}

# List endpoints
curl http://localhost:3000/__admin/endpoints
# [{"method": "GET", "path": "/api/users"}, ...]
```

#### Test Requirements
- [ ] Test YAML config parsing
- [ ] Test OpenAPI import
- [ ] Test path parameter matching
- [ ] Test conditional responses
- [ ] Test hot reload
- [ ] Test admin API

### Ticket 2: Dynamic Responses

**Summary:** Generate responses with Faker and templates.

**Definition of Done:** Responses vary realistically per request.

#### Acceptance Criteria

1. **Template Variables**
   - [ ] {{params.id}} - path parameters
   - [ ] {{query.search}} - query parameters
   - [ ] {{request.body.field}} - request body fields
   - [ ] {{headers.authorization}} - request headers

2. **Faker Integration**
   - [ ] {{faker.uuid}} - random UUID
   - [ ] {{faker.name}} - random name
   - [ ] {{faker.email}} - random email
   - [ ] {{faker.date.past}} - random past date
   - [ ] Support for all Faker methods

3. **Array Generation**
   - [ ] Generate array of N items
   - [ ] N configurable per request (?count=10)
   - [ ] Each item uses Faker for unique values

4. **Stateful Mocking**
   - [ ] Store data from POST requests
   - [ ] Return stored data in GET requests
   - [ ] Update stored data with PUT/PATCH
   - [ ] Delete stored data with DELETE
   - [ ] Persist state across restarts (optional)

5. **Delay Simulation**
   - [ ] Fixed delay per endpoint
   - [ ] Random delay range (100-500ms)
   - [ ] Delay based on request attributes

#### Demo Script
```yaml
# Dynamic user endpoint
- method: GET
  path: /api/users
  response:
    status: 200
    delay: "100-300"  # Random 100-300ms delay
    body:
      users:
        $repeat: "{{query.count | default: 10}}"
        item:
          id: "{{faker.uuid}}"
          name: "{{faker.name.fullName}}"
          email: "{{faker.internet.email}}"
          createdAt: "{{faker.date.past}}"

# Stateful CRUD
- method: POST
  path: /api/items
  stateful: true
  response:
    status: 201
    body:
      id: "{{faker.uuid}}"
      name: "{{request.body.name}}"

- method: GET
  path: /api/items
  stateful: true
  response:
    status: 200
    body: "{{state.items}}"  # Returns all stored items
```

```bash
# Get dynamic users (each call returns different data)
curl "http://localhost:3000/api/users?count=5"
# {"users": [{"id": "uuid-1", "name": "John Doe", ...}, ...]}

curl "http://localhost:3000/api/users?count=5"
# {"users": [{"id": "uuid-2", "name": "Jane Smith", ...}, ...]}  # Different!

# Stateful mock
curl -X POST http://localhost:3000/api/items -d '{"name": "Widget"}'
# {"id": "abc-123", "name": "Widget"}

curl http://localhost:3000/api/items
# [{"id": "abc-123", "name": "Widget"}]
```

#### Test Requirements
- [ ] Test template variable substitution
- [ ] Test all Faker methods used
- [ ] Test array generation
- [ ] Test stateful CRUD operations
- [ ] Test delay simulation accuracy
- [ ] Test state persistence

### Ticket 3: Request Recording

**Summary:** Record and replay real API traffic.

**Definition of Done:** Traffic recorded and replayed accurately.

#### Acceptance Criteria

1. **Proxy Mode**
   - [ ] Forward requests to real backend
   - [ ] Record request and response pairs
   - [ ] Transparent to client
   - [ ] Support HTTPS (with CA cert)

2. **Recording**
   - [ ] Record: method, URL, headers, body, response
   - [ ] Timestamp each recording
   - [ ] Filter sensitive headers (Authorization)
   - [ ] Storage: file per endpoint or single HAR file

3. **Replay Mode**
   - [ ] Serve recorded responses
   - [ ] Match by: method + URL path
   - [ ] Match by: method + URL + body hash
   - [ ] Fall through to proxy if no match

4. **Recording Management**
   - [ ] List all recordings
   - [ ] Delete specific recordings
   - [ ] Clear all recordings
   - [ ] Export as mock configuration

5. **Comparison**
   - [ ] Compare current response to recorded
   - [ ] Report differences
   - [ ] Useful for API regression testing

#### Demo Script
```bash
# Start in record mode
mock-server record --target https://api.example.com --port 3000 --output recordings/
# Proxying to https://api.example.com
# Recording to recordings/

# Make requests (they go to real API)
curl http://localhost:3000/api/users
# Real response from api.example.com, recorded to recordings/api_users_GET.json

curl -X POST http://localhost:3000/api/users -d '{"name": "Test"}'
# Real response recorded

# Switch to replay mode
mock-server replay --recordings recordings/ --port 3000
# Replaying from recordings/

# Same requests now return recorded responses
curl http://localhost:3000/api/users
# Returns recorded response (no network call)

# Export recordings as mock config
mock-server export --recordings recordings/ --output mocks/generated.yaml

# Compare mode (for testing)
mock-server compare --recordings recordings/ --target https://api.staging.example.com
# GET /api/users: MATCH
# POST /api/users: DIFF (response body changed)
```

#### Test Requirements
- [ ] Test proxy mode forwarding
- [ ] Test recording capture
- [ ] Test replay matching
- [ ] Test sensitive header filtering
- [ ] Test HAR export format
- [ ] Test comparison reporting
