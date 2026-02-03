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

### Ticket 2: Database Schema

**Summary:** Create PostgreSQL schema for tasks table.

**Definition of Done:** Schema is created and migrations run successfully.

### Ticket 3: CRUD Endpoints

**Summary:** Implement all CRUD operations for tasks.

**Definition of Done:** All endpoints return expected responses.
