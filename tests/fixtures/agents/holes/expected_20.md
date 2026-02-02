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

### Identified Weaknesses

1. **Comment preservation not addressed**: Comments aren't in AST; need special handling.

2. **Idempotency not guaranteed**: Formatting twice should produce same result.

3. **Parse error handling unclear**: What if code has syntax errors?

4. **No pragma/ignore support**: Users need to skip formatting specific code sections.

5. **Editor integration not planned**: VSCode, Vim, etc. integrations are critical for adoption.

### Edge Cases

- Template literals with complex expressions
- JSX/TSX with mixed content
- Decorators (multiple syntax variants)
- Very long strings/regexes (should they wrap?)
- Trailing commas in different contexts
- Preserving intentional alignment in object literals

### Assumptions to Validate

- What are the "different style choices" compared to Prettier?
- Should .editorconfig be respected?
- Is there a Prettier migration path (config compatibility)?
- Do we need range formatting (format selection only)?
- Is WASM target needed for browser/editor use?

### Potential Failures

- Tree-sitter grammar bugs on edge cases
- Loss of semantic meaning (changing code behavior)
- Infinite loops in formatting algorithm
- OOM on very large files
- File corruption if write fails mid-stream
- Encoding issues (BOM, non-UTF8 files)

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
