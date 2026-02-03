---
id: "test-001"
title: "REST API for Task Management"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a REST API for a task management application. Users can create, read, update, and delete tasks. The API will be built with Node.js and Express, using PostgreSQL for storage.

## Constraints

- Must be completed in 2 weeks
- Use existing company infrastructure

## Implementation Notes

- Set up Express server with standard middleware
- Create task model with fields: id, title, description, status, created_at
- Implement CRUD endpoints
- Connect to PostgreSQL database

## Review Notes

(none yet)

## Tickets

### Ticket 1: Project Setup

**Summary:** Initialize Node.js project with Express and necessary dependencies.

**Definition of Done:** Project runs locally with a hello world endpoint.

#### Acceptance Criteria

1. **Project Structure**
   - [ ] package.json exists with name, version, and scripts
   - [ ] tsconfig.json configured for ES2020 target
   - [ ] src/ directory structure created (routes/, models/, middleware/)
   - [ ] .env.example with required environment variables documented

2. **Dependencies Installed**
   - [ ] express, pg, dotenv in dependencies
   - [ ] typescript, @types/express, @types/node, ts-node, nodemon in devDependencies
   - [ ] All packages at latest stable versions

3. **Server Runs**
   - [ ] `npm run dev` starts server without errors
   - [ ] Server listens on PORT from environment (default 3000)
   - [ ] Console logs "Server running on port {PORT}"

4. **Hello World Endpoint**
   - [ ] GET / returns 200 status
   - [ ] Response body is `{"message": "Hello World", "status": "ok"}`
   - [ ] Content-Type header is application/json

#### Demo Script
```bash
# Start the server
npm run dev

# In another terminal, test the endpoint
curl -i http://localhost:3000/
# Expected: HTTP/1.1 200 OK, {"message":"Hello World","status":"ok"}
```

### Ticket 2: Database Schema

**Summary:** Create PostgreSQL schema for tasks table.

**Definition of Done:** Schema is created and migrations run successfully.

#### Acceptance Criteria

1. **Migration Tool Configured**
   - [ ] db-migrate or similar tool installed
   - [ ] database.json configured with connection settings
   - [ ] Migrations directory exists

2. **Tasks Table Schema**
   - [ ] Table named `tasks` exists
   - [ ] Column `id` is UUID with DEFAULT gen_random_uuid()
   - [ ] Column `title` is VARCHAR(255) NOT NULL
   - [ ] Column `description` is TEXT, nullable
   - [ ] Column `status` is VARCHAR(50) DEFAULT 'pending'
   - [ ] Column `created_at` is TIMESTAMPTZ DEFAULT NOW()
   - [ ] Column `updated_at` is TIMESTAMPTZ DEFAULT NOW()
   - [ ] Primary key on `id`

3. **Migration Execution**
   - [ ] `npm run migrate:up` applies migrations without error
   - [ ] `npm run migrate:down` rolls back without error
   - [ ] Running migrate:up twice is idempotent (no error)

4. **Database Connection**
   - [ ] Application connects to database on startup
   - [ ] Connection error logs helpful message with host/port
   - [ ] Health check endpoint includes database status

#### Demo Script
```bash
# Run migrations
npm run migrate:up

# Verify table exists
psql $DATABASE_URL -c "\d tasks"
# Expected: Table with all columns listed

# Test rollback
npm run migrate:down
npm run migrate:up
```

### Ticket 3: CRUD Endpoints

**Summary:** Implement all CRUD operations for tasks.

**Definition of Done:** All endpoints return expected responses.

#### Acceptance Criteria

1. **POST /tasks (Create)**
   - [ ] Accepts JSON body with title (required), description (optional), status (optional)
   - [ ] Returns 201 Created with full task object including generated id
   - [ ] Returns 400 Bad Request if title missing
   - [ ] Returns 400 Bad Request if title exceeds 255 characters

2. **GET /tasks (List)**
   - [ ] Returns 200 with array of all tasks
   - [ ] Tasks ordered by created_at DESC (newest first)
   - [ ] Empty array returned when no tasks exist
   - [ ] Response includes total count in header or body

3. **GET /tasks/:id (Read)**
   - [ ] Returns 200 with single task object
   - [ ] Returns 404 Not Found if task doesn't exist
   - [ ] Returns 400 Bad Request if id is not valid UUID

4. **PUT /tasks/:id (Update)**
   - [ ] Accepts JSON body with title, description, status (all optional)
   - [ ] Returns 200 with updated task object
   - [ ] Updates `updated_at` timestamp
   - [ ] Returns 404 Not Found if task doesn't exist
   - [ ] Returns 400 if no fields provided

5. **DELETE /tasks/:id (Delete)**
   - [ ] Returns 204 No Content on success
   - [ ] Returns 404 Not Found if task doesn't exist
   - [ ] Task is permanently removed from database

6. **Error Handling**
   - [ ] All errors return JSON with `error` field
   - [ ] 500 errors don't expose internal details
   - [ ] Request validation errors list all invalid fields

#### Demo Script
```bash
# Create a task
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Test Task", "description": "A test"}'
# Expected: 201, {"id": "uuid", "title": "Test Task", ...}

# List tasks
curl http://localhost:3000/tasks
# Expected: 200, [{"id": "uuid", ...}]

# Get single task
curl http://localhost:3000/tasks/{id}
# Expected: 200, {"id": "uuid", ...}

# Update task
curl -X PUT http://localhost:3000/tasks/{id} \
  -H "Content-Type: application/json" \
  -d '{"status": "completed"}'
# Expected: 200, {"id": "uuid", "status": "completed", ...}

# Delete task
curl -X DELETE http://localhost:3000/tasks/{id}
# Expected: 204 No Content

# Verify deletion
curl http://localhost:3000/tasks/{id}
# Expected: 404 Not Found
```

#### Test Requirements
- [ ] Unit tests for request validation
- [ ] Integration tests for each endpoint
- [ ] Test coverage > 80%
- [ ] All tests pass in CI pipeline
