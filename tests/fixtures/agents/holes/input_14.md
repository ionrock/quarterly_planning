---
id: "test-014"
title: "Git Hooks Manager"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a CLI tool that manages Git hooks across repositories. Users define hooks in a config file, and the tool installs/updates them. Supports pre-commit, commit-msg, and pre-push hooks. Written in Rust.

## Constraints

- Must work with any Git repository
- No runtime dependencies beyond Git

## Implementation Notes

- Read hook configuration from .hooks.toml
- Generate shell scripts for each hook type
- Support running multiple commands per hook
- Provide install and uninstall commands

## Review Notes

(none yet)

## Tickets

### Ticket 1: Config Parser

**Summary:** Parse .hooks.toml configuration file.

**Definition of Done:** Configuration is correctly parsed into structs.

### Ticket 2: Hook Generator

**Summary:** Generate shell scripts from hook configuration.

**Definition of Done:** Generated scripts execute configured commands.

### Ticket 3: CLI Commands

**Summary:** Implement install, uninstall, and run commands.

**Definition of Done:** Hooks are correctly installed in .git/hooks.
