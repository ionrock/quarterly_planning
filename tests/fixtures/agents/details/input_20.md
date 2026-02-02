---
id: "test-020"
title: "Code Formatter"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create an opinionated code formatter for JavaScript/TypeScript. Parses code into AST, applies formatting rules, and outputs formatted code. Similar to Prettier but with different style choices. Written in Rust for speed.

## Constraints

- Format 10,000 lines per second
- Support ES2024 and TypeScript 5.x syntax

## Implementation Notes

- Use tree-sitter for parsing
- Wadler-Lindig pretty printing algorithm
- Configurable via .formatter.json
- Support stdin/stdout and file in-place formatting

## Review Notes

(none yet)

## Tickets

### Ticket 1: Parser Integration

**Summary:** Integrate tree-sitter for JS/TS parsing.

**Definition of Done:** Code is parsed into AST correctly.

### Ticket 2: Formatting Engine

**Summary:** Implement pretty printing algorithm.

**Definition of Done:** Code is formatted according to rules.

### Ticket 3: CLI Interface

**Summary:** Build CLI for file and stdin formatting.

**Definition of Done:** Files can be formatted in place or to stdout.
