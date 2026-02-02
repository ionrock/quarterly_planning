---
id: "test-005"
title: "Database Migration Tool"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a database migration tool that tracks and applies schema changes. Supports versioned migration files, rollback capability, and works with PostgreSQL. Written in Rust for performance and reliability.

## Constraints

- Must be safe to run in production
- Support both up and down migrations

## Implementation Notes

### Technology Stack
- **Language:** Rust 1.75+
- **Database Driver:** tokio-postgres with deadpool for connection pooling
- **CLI Framework:** clap v4 with derive macros
- **Config:** config-rs for environment/file configuration

### Migration File Format
```sql
-- migrations/20260115120000_create_users.sql

-- migrate:up
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);

-- migrate:down
DROP TABLE IF EXISTS users;
```

### Migration Tracking Schema
```sql
CREATE TABLE IF NOT EXISTS _migrations (
    version VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    checksum VARCHAR(64) NOT NULL,
    execution_time_ms INTEGER
);
```

### Core Data Structures
```rust
#[derive(Debug, Clone)]
pub struct Migration {
    pub version: String,      // "20260115120000"
    pub name: String,         // "create_users"
    pub up_sql: String,
    pub down_sql: Option<String>,
    pub checksum: String,     // SHA-256 of file content
}

#[derive(Debug)]
pub struct AppliedMigration {
    pub version: String,
    pub name: String,
    pub applied_at: DateTime<Utc>,
    pub checksum: String,
    pub execution_time_ms: i32,
}

pub enum MigrationDirection {
    Up,
    Down,
}
```

### CLI Interface
```
migrate [OPTIONS] <COMMAND>

Commands:
  up        Apply pending migrations
  down      Rollback last N migrations
  status    Show migration status
  create    Create a new migration file
  validate  Check migrations for issues

Options:
  -c, --config <FILE>     Config file path
  -d, --database <URL>    Database URL (overrides config)
  --dry-run               Show SQL without executing
  -n, --count <N>         Number of migrations to apply/rollback
  -v, --verbose           Verbose output
```

### Migration Execution Algorithm
```rust
pub async fn apply_migrations(
    pool: &Pool,
    pending: Vec<Migration>,
    dry_run: bool,
) -> Result<Vec<MigrationResult>> {
    let mut results = Vec::new();

    for migration in pending {
        let start = Instant::now();

        if dry_run {
            println!("-- Would apply: {}", migration.version);
            println!("{}", migration.up_sql);
            continue;
        }

        let mut client = pool.get().await?;
        let tx = client.transaction().await?;

        // Execute migration SQL
        tx.batch_execute(&migration.up_sql).await
            .map_err(|e| MigrationError::ExecutionFailed {
                version: migration.version.clone(),
                cause: e,
            })?;

        // Record in tracking table
        tx.execute(
            "INSERT INTO _migrations (version, name, checksum, execution_time_ms) VALUES ($1, $2, $3, $4)",
            &[&migration.version, &migration.name, &migration.checksum, &(start.elapsed().as_millis() as i32)],
        ).await?;

        tx.commit().await?;

        results.push(MigrationResult {
            version: migration.version,
            duration: start.elapsed(),
            status: MigrationStatus::Applied,
        });
    }

    Ok(results)
}
```

### Checksum Calculation
```rust
fn calculate_checksum(content: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}
```

### Advisory Lock for Concurrent Safety
```rust
const MIGRATION_LOCK_ID: i64 = 0x4d494752; // "MIGR" in hex

pub async fn with_migration_lock<F, T>(pool: &Pool, f: F) -> Result<T>
where
    F: FnOnce() -> Future<Output = Result<T>>,
{
    let client = pool.get().await?;

    // Acquire advisory lock
    client.execute(
        "SELECT pg_advisory_lock($1)",
        &[&MIGRATION_LOCK_ID]
    ).await?;

    let result = f().await;

    // Release lock
    client.execute(
        "SELECT pg_advisory_unlock($1)",
        &[&MIGRATION_LOCK_ID]
    ).await?;

    result
}
```

### Configuration
```toml
# migrate.toml
[database]
url = "${DATABASE_URL}"
max_connections = 5
connect_timeout_secs = 10

[migrations]
directory = "./migrations"
table_name = "_migrations"
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: Migration File Parser

**Summary:** Parse SQL migration files and extract up/down sections.

**Definition of Done:** Parser correctly identifies migration content.

### Ticket 2: Migration Tracking

**Summary:** Create and manage the migrations tracking table.

**Definition of Done:** Applied migrations are recorded and queryable.

### Ticket 3: CLI Commands

**Summary:** Implement migrate, rollback, and status commands.

**Definition of Done:** All commands work as expected.
