---
id: "test-010"
title: "Database Migration Tool"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a database migration tool supporting schema changes across multiple database types. Tracks migration history, supports rollbacks, and generates migration files from schema diffs.

## Constraints

- Support PostgreSQL, MySQL, and SQLite
- Migrations must be atomic (all or nothing)

## Implementation Notes

- Go CLI tool
- Embedded migration runner
- SQL and Go-based migrations

## Review Notes

(none yet)

## Tickets

### Ticket 1: Migration Runner

**Summary:** Execute migrations in order with tracking.

**Definition of Done:** Migrations run reliably with history tracking.

### Ticket 2: Rollback Support

**Summary:** Reverse migrations with down scripts.

**Definition of Done:** Any migration can be rolled back.

### Ticket 3: Schema Diff

**Summary:** Generate migration from schema changes.

**Definition of Done:** Diff detects common schema changes.
