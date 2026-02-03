---
id: "test-005"
title: "Database Migration Tool"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a database migration tool that manages schema changes across environments. Supports up/down migrations, tracks applied migrations, and handles migration dependencies.

## Constraints

- Must support PostgreSQL and MySQL
- Rollback must be atomic

## Implementation Notes

- Written in Rust for performance
- SQL-based migration files
- Checksum verification for migration integrity

## Review Notes

(none yet)

## Tickets

### Ticket 1: Migration File Format

**Summary:** Define and parse migration file format.

**Definition of Done:** Can read migration files with up/down sections.

#### Steps

1. **Create Rust project structure**
   - Run `cargo new migrate --bin`
   - Add lib.rs for library code
   - Verify: `cargo build` succeeds

2. **Define Migration struct**
   - Create src/migration.rs
   - Define struct with version, name, up_sql, down_sql, checksum fields
   - Verify: struct compiles

3. **Define migration file naming convention**
   - Format: {version}_{name}.sql (e.g., 001_create_users.sql)
   - Extract version and name from filename
   - Verify: parser extracts components correctly

4. **Create migration file parser**
   - Create src/parser.rs
   - Look for `-- migrate:up` and `-- migrate:down` markers
   - Split content into up and down sections
   - Verify: parses sample migration file

5. **Implement checksum calculation**
   - Use SHA256 hash of file contents
   - Store as hex string
   - Verify: same file produces same checksum

6. **Handle missing down migration**
   - Allow migrations without down section
   - Return error on rollback attempt
   - Verify: up-only migration parses successfully

7. **Create migration loader**
   - Create src/loader.rs
   - Scan migrations directory for .sql files
   - Parse each file into Migration struct
   - Verify: loads all migrations from directory

8. **Sort migrations by version**
   - Sort migration list by version number
   - Detect version gaps and warn
   - Verify: migrations in correct order

### Ticket 2: Migration Tracking

**Summary:** Track which migrations have been applied.

**Definition of Done:** Applied migrations stored in database table.

#### Steps

1. **Add database dependencies**
   - Add sqlx with postgres and mysql features to Cargo.toml
   - Add tokio for async runtime
   - Verify: dependencies resolve

2. **Create database connection abstraction**
   - Create src/database.rs with Database trait
   - Define execute(sql), query_one(sql), query_all(sql) methods
   - Verify: trait compiles

3. **Implement PostgreSQL connection**
   - Create src/database/postgres.rs
   - Implement Database trait using sqlx::PgPool
   - Verify: connects to PostgreSQL

4. **Implement MySQL connection**
   - Create src/database/mysql.rs
   - Implement Database trait using sqlx::MySqlPool
   - Verify: connects to MySQL

5. **Create schema_migrations table DDL**
   - Columns: version (VARCHAR PRIMARY KEY), name, checksum, applied_at
   - Use IF NOT EXISTS for idempotency
   - Verify: DDL is valid for both databases

6. **Implement ensure_migrations_table**
   - Create table if not exists
   - Call on every migration run
   - Verify: table created on first run

7. **Implement get_applied_migrations**
   - Query schema_migrations table
   - Return list of applied version numbers
   - Verify: returns empty list for fresh database

8. **Implement record_migration**
   - Insert row into schema_migrations
   - Include version, name, checksum, current timestamp
   - Verify: row inserted after migration

9. **Implement remove_migration_record**
   - Delete row from schema_migrations by version
   - Called after successful rollback
   - Verify: row removed after rollback

10. **Implement verify_checksum**
    - Compare stored checksum with current file checksum
    - Error if mismatch (migration modified after apply)
    - Verify: detects modified migrations

### Ticket 3: Migration Execution

**Summary:** Execute migrations up and down.

**Definition of Done:** Migrations apply and rollback correctly.

#### Steps

1. **Create migrator module**
   - Create src/migrator.rs
   - Accept database and migrations list
   - Verify: module structure compiles

2. **Calculate pending migrations**
   - Compare loaded migrations with applied migrations
   - Return list of migrations to apply
   - Verify: correctly identifies pending migrations

3. **Implement migrate_up for single migration**
   - Begin transaction
   - Execute up_sql
   - Record migration
   - Commit transaction
   - Verify: migration applied atomically

4. **Implement migrate_up_all**
   - Apply all pending migrations in version order
   - Stop on first failure
   - Report progress to stdout
   - Verify: all pending migrations applied

5. **Implement migrate_down for single migration**
   - Begin transaction
   - Execute down_sql
   - Remove migration record
   - Commit transaction
   - Verify: migration rolled back atomically

6. **Implement migrate_down_to**
   - Rollback migrations until target version
   - Rollback in reverse version order
   - Verify: rolls back to specific version

7. **Add --dry-run flag**
   - Print SQL that would be executed
   - Don't actually execute
   - Verify: shows migration SQL without applying

8. **Handle transaction errors**
   - Catch SQL errors during execution
   - Rollback transaction on error
   - Report error with context
   - Verify: failed migration doesn't leave partial state

9. **Add migration locking**
   - Use advisory lock to prevent concurrent migrations
   - PostgreSQL: pg_advisory_lock
   - MySQL: GET_LOCK
   - Verify: second migration waits for first

10. **Create CLI commands**
    - `migrate up` - apply all pending
    - `migrate down` - rollback last migration
    - `migrate status` - show migration status
    - Verify: all commands work

11. **Add verbose output option**
    - `-v` flag for detailed output
    - Show SQL being executed
    - Show timing information
    - Verify: verbose mode shows details
