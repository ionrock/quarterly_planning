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

#### Acceptance Criteria

1. **Migration Discovery**
   - [ ] Scan migrations directory for .sql and .go files
   - [ ] Filename format: {version}_{name}.{up|down}.sql
   - [ ] Version is numeric timestamp (20240115100000)
   - [ ] Migrations sorted by version ascending

2. **Migration Execution**
   - [ ] `migrate up` runs all pending migrations
   - [ ] `migrate up N` runs next N migrations
   - [ ] Each migration runs in a transaction
   - [ ] Failed migration rolls back transaction
   - [ ] Execution stops on first failure

3. **History Tracking**
   - [ ] schema_migrations table tracks applied migrations
   - [ ] Columns: version, name, applied_at, checksum
   - [ ] Table created automatically if not exists
   - [ ] Checksum detects modified migrations

4. **Database Support**
   - [ ] PostgreSQL: full transaction support
   - [ ] MySQL: DDL not transactional (warning shown)
   - [ ] SQLite: single-connection transaction
   - [ ] Connection string from --database flag or DATABASE_URL

5. **Output**
   - [ ] Progress shown for each migration
   - [ ] Duration logged for each migration
   - [ ] Final summary of applied/failed/skipped
   - [ ] --quiet flag suppresses output

#### Demo Script
```bash
# Initialize (create schema_migrations table)
migrate init --database "postgres://localhost/mydb"
# Created schema_migrations table

# Check status
migrate status --database "postgres://localhost/mydb"
# VERSION          NAME                  STATUS      APPLIED AT
# 20240115100000   create_users          applied     2024-01-15 10:00:00
# 20240115110000   add_email_to_users    pending     -
# 20240115120000   create_orders         pending     -

# Run migrations
migrate up --database "postgres://localhost/mydb"
# Applying 20240115110000_add_email_to_users... done (15ms)
# Applying 20240115120000_create_orders... done (22ms)
# Applied 2 migrations

# Run specific count
migrate up 1 --database "postgres://localhost/mydb"
# Applying 20240115130000_add_index... done (5ms)
# Applied 1 migration

# Dry run
migrate up --dry-run --database "postgres://localhost/mydb"
# Would apply: 20240115140000_create_products
# Would apply: 20240115150000_add_sku
```

#### Test Requirements
- [ ] Test migration discovery and ordering
- [ ] Test transaction rollback on failure
- [ ] Test with each supported database
- [ ] Test checksum validation
- [ ] Test concurrent migration attempts (locking)

### Ticket 2: Rollback Support

**Summary:** Reverse migrations with down scripts.

**Definition of Done:** Any migration can be rolled back.

#### Acceptance Criteria

1. **Down Migrations**
   - [ ] Each up migration has corresponding down migration
   - [ ] Down migration reverses schema changes
   - [ ] `migrate down` rolls back last migration
   - [ ] `migrate down N` rolls back last N migrations

2. **Rollback to Version**
   - [ ] `migrate down --to VERSION` rolls back to specific version
   - [ ] All migrations after VERSION are rolled back
   - [ ] Migration at VERSION remains applied

3. **Rollback Validation**
   - [ ] Warn if down migration is missing
   - [ ] --force flag to skip missing down migrations
   - [ ] Checksum verified before rollback

4. **Transaction Handling**
   - [ ] Rollback is atomic
   - [ ] Failed rollback re-applies migration
   - [ ] Partial rollback state avoided

5. **Fresh Migration**
   - [ ] `migrate fresh` drops all tables and re-runs
   - [ ] Requires --confirm flag in production
   - [ ] Useful for development/testing

#### Demo Script
```bash
# Rollback last migration
migrate down --database "postgres://localhost/mydb"
# Rolling back 20240115120000_create_orders... done (10ms)
# Rolled back 1 migration

# Rollback multiple
migrate down 2 --database "postgres://localhost/mydb"
# Rolling back 20240115110000_add_email_to_users... done (8ms)
# Rolling back 20240115100000_create_users... done (12ms)
# Rolled back 2 migrations

# Rollback to specific version
migrate down --to 20240115100000 --database "postgres://localhost/mydb"
# Rolling back 20240115120000_create_orders... done
# Rolling back 20240115110000_add_email_to_users... done
# Rolled back to version 20240115100000

# Fresh start
migrate fresh --database "postgres://localhost/mydb" --confirm
# Dropping all tables...
# Running all migrations...
# Applied 5 migrations
```

#### Test Requirements
- [ ] Test single migration rollback
- [ ] Test multi-migration rollback
- [ ] Test rollback to version
- [ ] Test failed rollback recovery
- [ ] Test missing down migration warning
- [ ] Test fresh with confirmation

### Ticket 3: Schema Diff

**Summary:** Generate migration from schema changes.

**Definition of Done:** Diff detects common schema changes.

#### Acceptance Criteria

1. **Diff Command**
   - [ ] `migrate diff` compares two schemas
   - [ ] Compare: current DB vs. schema file
   - [ ] Compare: current DB vs. another DB
   - [ ] Output: SQL migration or structured diff

2. **Detected Changes**
   - [ ] New tables
   - [ ] Dropped tables
   - [ ] New columns
   - [ ] Dropped columns
   - [ ] Column type changes
   - [ ] New indexes
   - [ ] Dropped indexes
   - [ ] Foreign key changes

3. **Migration Generation**
   - [ ] `migrate diff --out FILE` writes migration file
   - [ ] Generates both up and down migrations
   - [ ] Respects column order
   - [ ] Includes data migration hints (comments)

4. **Schema File**
   - [ ] Support .sql schema definition file
   - [ ] Support multiple files (one per table)
   - [ ] `migrate dump` exports current schema

5. **Safety**
   - [ ] Destructive changes require --allow-destructive
   - [ ] Warns about data loss (drop column/table)
   - [ ] Suggests safe alternatives (rename vs drop+add)

#### Demo Script
```bash
# Dump current schema
migrate dump --database "postgres://localhost/mydb" > schema.sql

# Modify schema.sql (add column, change type, etc.)

# Generate diff
migrate diff --database "postgres://localhost/mydb" --schema schema.sql
# Diff:
# + ADD COLUMN users.phone VARCHAR(20)
# ~ ALTER COLUMN users.name VARCHAR(100) -> VARCHAR(200)
# - DROP INDEX idx_users_email
# ! DROP TABLE old_logs (DESTRUCTIVE)

# Generate migration file
migrate diff --database "postgres://localhost/mydb" --schema schema.sql \
  --out migrations/20240115160000_schema_update.sql

# Review generated migration
cat migrations/20240115160000_schema_update.up.sql
# -- +migrate Up
# ALTER TABLE users ADD COLUMN phone VARCHAR(20);
# ALTER TABLE users ALTER COLUMN name TYPE VARCHAR(200);
# DROP INDEX idx_users_email;
# -- WARNING: Dropping table old_logs will delete all data
# -- DROP TABLE old_logs;

cat migrations/20240115160000_schema_update.down.sql
# -- +migrate Down
# -- CREATE TABLE old_logs (...); -- Cannot restore deleted data
# CREATE INDEX idx_users_email ON users(email);
# ALTER TABLE users ALTER COLUMN name TYPE VARCHAR(100);
# ALTER TABLE users DROP COLUMN phone;
```

#### Test Requirements
- [ ] Test detection of each change type
- [ ] Test migration file generation
- [ ] Test destructive change warnings
- [ ] Test with each supported database
- [ ] Test round-trip (diff -> apply -> diff = no changes)
