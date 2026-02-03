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

#### Steps

1. **Create project directory and initialize npm**
   - Run `npm init -y` to create package.json
   - Verify: package.json exists with valid JSON

2. **Install core dependencies**
   - Run `npm install express pg dotenv`
   - Verify: node_modules created, dependencies in package.json

3. **Install dev dependencies**
   - Run `npm install -D typescript @types/express @types/node ts-node nodemon`
   - Verify: devDependencies section populated

4. **Create TypeScript configuration**
   - Create tsconfig.json with target ES2020, module commonjs
   - Verify: `npx tsc --noEmit` runs without errors

5. **Create project directory structure**
   - Create src/, src/routes/, src/models/, src/middleware/ directories
   - Verify: directories exist

6. **Create entry point with hello world**
   - Create src/index.ts with Express app listening on PORT
   - Add GET / route returning { message: "Hello World" }
   - Verify: `curl localhost:3000` returns JSON response

7. **Add npm scripts**
   - Add "dev": "nodemon src/index.ts" to package.json
   - Add "build": "tsc" and "start": "node dist/index.js"
   - Verify: `npm run dev` starts server with hot reload

### Ticket 2: Database Schema

**Summary:** Create PostgreSQL schema for tasks table.

**Definition of Done:** Schema is created and migrations run successfully.

#### Steps

1. **Install database migration tool**
   - Run `npm install -D db-migrate db-migrate-pg`
   - Verify: packages added to devDependencies

2. **Initialize db-migrate**
   - Run `npx db-migrate init`
   - Verify: database.json and migrations/ directory created

3. **Configure database connection**
   - Create database.json with dev/test/prod environments
   - Read connection string from DATABASE_URL env var
   - Verify: database.json has correct structure

4. **Create tasks table migration**
   - Run `npx db-migrate create create-tasks-table`
   - Verify: migration file created in migrations/

5. **Write up migration SQL**
   - Add CREATE TABLE tasks with columns: id (UUID PRIMARY KEY DEFAULT gen_random_uuid()), title (VARCHAR(255) NOT NULL), description (TEXT), status (VARCHAR(50) DEFAULT 'pending'), created_at (TIMESTAMPTZ DEFAULT NOW()), updated_at (TIMESTAMPTZ DEFAULT NOW())
   - Verify: SQL syntax is valid

6. **Write down migration SQL**
   - Add DROP TABLE IF EXISTS tasks
   - Verify: rollback SQL is valid

7. **Run migration against local database**
   - Ensure PostgreSQL is running locally
   - Run `npx db-migrate up`
   - Verify: tasks table exists with `\d tasks` in psql

8. **Test migration rollback**
   - Run `npx db-migrate down`
   - Verify: tasks table no longer exists
   - Run `npx db-migrate up` to restore

### Ticket 3: CRUD Endpoints

**Summary:** Implement all CRUD operations for tasks.

**Definition of Done:** All endpoints return expected responses.

#### Steps

1. **Create database connection pool**
   - Create src/db.ts with pg Pool configured from DATABASE_URL
   - Export pool for use in routes
   - Verify: pool.query('SELECT 1') succeeds

2. **Create Task type definition**
   - Create src/types/task.ts with Task interface
   - Include id, title, description, status, created_at, updated_at fields
   - Verify: TypeScript compiles without errors

3. **Create tasks router file**
   - Create src/routes/tasks.ts with Express Router
   - Verify: router exports without errors

4. **Implement POST /tasks (create)**
   - Accept { title, description?, status? } in request body
   - Insert into database, return created task with 201 status
   - Verify: `curl -X POST -H "Content-Type: application/json" -d '{"title":"Test"}' localhost:3000/tasks` returns created task

5. **Implement GET /tasks (list all)**
   - Query all tasks from database ordered by created_at DESC
   - Return array of tasks with 200 status
   - Verify: `curl localhost:3000/tasks` returns array

6. **Implement GET /tasks/:id (get one)**
   - Query task by id from database
   - Return task with 200, or 404 if not found
   - Verify: `curl localhost:3000/tasks/{id}` returns single task

7. **Implement PUT /tasks/:id (update)**
   - Accept { title?, description?, status? } in request body
   - Update task in database, set updated_at to NOW()
   - Return updated task with 200, or 404 if not found
   - Verify: `curl -X PUT -H "Content-Type: application/json" -d '{"status":"done"}' localhost:3000/tasks/{id}` returns updated task

8. **Implement DELETE /tasks/:id (delete)**
   - Delete task from database by id
   - Return 204 on success, or 404 if not found
   - Verify: `curl -X DELETE localhost:3000/tasks/{id}` returns 204

9. **Mount router in main app**
   - Import tasks router in src/index.ts
   - Mount at /tasks path
   - Verify: all endpoints accessible at /tasks/*

10. **Add request body parsing middleware**
    - Add express.json() middleware before routes
    - Verify: POST/PUT requests parse JSON body correctly

11. **Add basic error handling middleware**
    - Create error handler that catches exceptions
    - Return { error: message } with appropriate status code
    - Verify: invalid requests return proper error responses
