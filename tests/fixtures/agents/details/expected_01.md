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

### Technology Stack
- **Runtime:** Node.js 20 LTS
- **Framework:** Express 4.x with TypeScript
- **Database:** PostgreSQL 15+ with pg driver
- **Validation:** Zod for request/response validation
- **Testing:** Jest + Supertest for API tests

### API Endpoints
```
GET    /api/v1/tasks          - List all tasks (paginated)
GET    /api/v1/tasks/:id      - Get task by ID
POST   /api/v1/tasks          - Create new task
PUT    /api/v1/tasks/:id      - Update task
DELETE /api/v1/tasks/:id      - Delete task
```

### Data Model
```sql
CREATE TABLE tasks (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  title VARCHAR(255) NOT NULL,
  description TEXT,
  status VARCHAR(50) NOT NULL DEFAULT 'pending',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT valid_status CHECK (status IN ('pending', 'in_progress', 'completed', 'cancelled'))
);

CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_created_at ON tasks(created_at DESC);
```

### Request/Response Formats
```typescript
// Create/Update Request
interface TaskInput {
  title: string;        // Required, 1-255 chars
  description?: string; // Optional, max 10000 chars
  status?: TaskStatus;  // Optional on create, defaults to 'pending'
}

// Response
interface Task {
  id: string;
  title: string;
  description: string | null;
  status: 'pending' | 'in_progress' | 'completed' | 'cancelled';
  createdAt: string;  // ISO 8601
  updatedAt: string;  // ISO 8601
}

// List Response
interface TaskList {
  data: Task[];
  pagination: {
    total: number;
    page: number;
    pageSize: number;
    totalPages: number;
  };
}
```

### Error Handling
- Return RFC 7807 Problem Details format for errors
- HTTP 400 for validation errors with field-level details
- HTTP 404 for missing resources
- HTTP 500 for unexpected errors (log details, return generic message)

### Middleware Stack
1. `express.json()` - Parse JSON bodies (limit: 100kb)
2. `cors()` - Enable CORS for development
3. `helmet()` - Security headers
4. Request ID middleware - Add X-Request-ID header
5. Request logging middleware - Log method, path, duration

## Review Notes

(none yet)

## Tickets

### Ticket 1: Project Setup

**Summary:** Initialize Node.js project with Express and necessary dependencies.

**Definition of Done:** Project runs locally with a hello world endpoint.

### Ticket 2: Database Schema

**Summary:** Create PostgreSQL schema for tasks table.

**Definition of Done:** Schema is created and migrations run successfully.

### Ticket 3: CRUD Endpoints

**Summary:** Implement all CRUD operations for tasks.

**Definition of Done:** All endpoints return expected responses.
