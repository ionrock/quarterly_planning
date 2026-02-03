---
id: "test-002"
title: "CLI Password Manager"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a command-line password manager in Rust. Supports storing, retrieving, and generating passwords. Uses AES-256 encryption with a master password.

## Constraints

- Single binary with no external dependencies at runtime
- Must work offline

## Implementation Notes

- Use ring for cryptography
- SQLite for local storage
- Clipboard integration for copying passwords

## Review Notes

(none yet)

## Tickets

### Ticket 1: Encryption Core

**Summary:** Implement AES-256 encryption/decryption with key derivation.

**Definition of Done:** Can encrypt and decrypt arbitrary data.

### Ticket 2: Password Storage

**Summary:** Store and retrieve encrypted passwords in SQLite.

**Definition of Done:** CRUD operations work correctly.

### Ticket 3: CLI Interface

**Summary:** Implement command-line interface with subcommands.

**Definition of Done:** All commands work as documented.
