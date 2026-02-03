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

#### Acceptance Criteria

1. **Key Derivation**
   - [ ] PBKDF2 with SHA-256 used for key derivation
   - [ ] Minimum 100,000 iterations
   - [ ] 32-byte salt generated per vault
   - [ ] Derived key is 256 bits

2. **Encryption**
   - [ ] AES-256-GCM used for encryption
   - [ ] 96-bit nonce generated per encryption
   - [ ] Authentication tag verified on decryption
   - [ ] Ciphertext includes nonce and tag

3. **Security Properties**
   - [ ] Plaintext zeroed from memory after use
   - [ ] Wrong password returns authentication error
   - [ ] Tampered ciphertext detected and rejected

4. **API**
   - [ ] encrypt(plaintext, password) -> Result<Vec<u8>>
   - [ ] decrypt(ciphertext, password) -> Result<Vec<u8>>
   - [ ] Both functions are deterministic given same nonce

#### Demo Script
```bash
# Run encryption unit tests
cargo test encryption --release

# Interactive test
echo "test data" | cargo run -- encrypt --password "master123"
# Output: base64-encoded ciphertext

echo "<ciphertext>" | cargo run -- decrypt --password "master123"
# Output: test data

# Wrong password test
echo "<ciphertext>" | cargo run -- decrypt --password "wrong"
# Output: Error: Authentication failed
```

#### Test Requirements
- [ ] Unit tests for encrypt/decrypt round-trip
- [ ] Test wrong password rejection
- [ ] Test tampered ciphertext detection
- [ ] Benchmark key derivation time (should be >100ms)

### Ticket 2: Password Storage

**Summary:** Store and retrieve encrypted passwords in SQLite.

**Definition of Done:** CRUD operations work correctly.

#### Acceptance Criteria

1. **Database Schema**
   - [ ] Table `entries` with id, name, username, password_encrypted, url, notes, created_at, updated_at
   - [ ] Table `vault_meta` with salt, iterations, created_at
   - [ ] Index on entries.name for fast lookup

2. **Create Entry**
   - [ ] Entry created with unique name
   - [ ] Password encrypted before storage
   - [ ] Duplicate name returns error
   - [ ] Timestamps set automatically

3. **Read Entry**
   - [ ] Entry retrieved by name (case-insensitive)
   - [ ] Password decrypted on retrieval
   - [ ] Non-existent entry returns None
   - [ ] List all entries (names only, no passwords)

4. **Update Entry**
   - [ ] Any field can be updated
   - [ ] updated_at timestamp refreshed
   - [ ] Non-existent entry returns error

5. **Delete Entry**
   - [ ] Entry removed by name
   - [ ] Deletion is permanent
   - [ ] Non-existent entry returns error

#### Demo Script
```bash
# Initialize vault
cargo run -- init
# Prompts for master password

# Add entry
cargo run -- add github --username "user@example.com" --generate
# Output: Entry 'github' created. Password: <generated>

# Get entry
cargo run -- get github
# Output: Username: user@example.com, Password: <decrypted>

# List entries
cargo run -- list
# Output: github, aws, email (names only)

# Update entry
cargo run -- update github --username "newuser@example.com"
# Output: Entry 'github' updated

# Delete entry
cargo run -- delete github --confirm
# Output: Entry 'github' deleted
```

#### Test Requirements
- [ ] Integration tests with temporary database
- [ ] Test CRUD operations
- [ ] Test duplicate name handling
- [ ] Test case-insensitive lookup

### Ticket 3: CLI Interface

**Summary:** Implement command-line interface with subcommands.

**Definition of Done:** All commands work as documented.

#### Acceptance Criteria

1. **Init Command**
   - [ ] `pwm init` creates new vault at ~/.pwm/vault.db
   - [ ] Prompts for master password (twice for confirmation)
   - [ ] Fails if vault already exists (unless --force)
   - [ ] Creates ~/.pwm directory if needed

2. **Add Command**
   - [ ] `pwm add <name>` adds new entry
   - [ ] --username, --password, --url, --notes flags
   - [ ] --generate flag creates random password
   - [ ] --length flag sets generated password length (default 20)
   - [ ] --copy flag copies password to clipboard

3. **Get Command**
   - [ ] `pwm get <name>` retrieves entry
   - [ ] --copy flag copies password to clipboard
   - [ ] --show flag displays password (hidden by default)
   - [ ] Prompts for master password

4. **List Command**
   - [ ] `pwm list` shows all entry names
   - [ ] --verbose shows usernames and URLs too
   - [ ] Sorted alphabetically

5. **Generate Command**
   - [ ] `pwm generate` outputs random password
   - [ ] --length, --no-symbols, --no-numbers flags
   - [ ] --copy flag copies to clipboard

6. **Error Handling**
   - [ ] Invalid command shows help
   - [ ] Missing required args shows usage
   - [ ] Errors printed to stderr with exit code 1

#### Demo Script
```bash
# Full workflow
pwm init
# Enter master password: ****
# Confirm master password: ****
# Vault created at ~/.pwm/vault.db

pwm add github --username "me@example.com" --generate --copy
# Password generated and copied to clipboard
# Entry 'github' created

pwm get github --show
# Enter master password: ****
# Name: github
# Username: me@example.com
# Password: xK9#mP2$vL5@nQ8

pwm list
# github

pwm generate --length 32
# Output: aB3$kL9#mP2@vN5&xQ8*zR1!wT4%yU7
```

#### Test Requirements
- [ ] CLI integration tests using assert_cmd
- [ ] Test each subcommand
- [ ] Test error messages
- [ ] Test --help output for each command
