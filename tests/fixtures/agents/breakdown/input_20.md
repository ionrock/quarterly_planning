---
id: "test-020"
title: "Code Formatter"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a code formatter for a custom configuration language. Handles indentation, line wrapping, and comment alignment. Preserves semantic meaning while improving readability.

## Constraints

- Format 10,000 line file under 1 second
- Idempotent (formatting twice gives same result)

## Implementation Notes

- Written in Rust
- Parse-print architecture
- Configurable style options

## Review Notes

(none yet)

## Tickets

### Ticket 1: Parser

**Summary:** Parse configuration language to AST.

**Definition of Done:** All valid configs parse correctly.

### Ticket 2: Formatter

**Summary:** Format AST back to source.

**Definition of Done:** Output is well-formatted.

### Ticket 3: CLI Tool

**Summary:** Create command-line interface.

**Definition of Done:** Tool formats files in place or to stdout.
