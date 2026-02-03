---
id: "test-016"
title: "Markdown Parser"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a CommonMark-compliant Markdown parser that converts Markdown to HTML. Supports extensions for tables, task lists, and syntax highlighting.

## Constraints

- Full CommonMark compliance
- Parse 1MB document under 100ms

## Implementation Notes

- Written in Rust for performance
- Two-pass parsing (blocks then inlines)
- Extension system for custom syntax

## Review Notes

(none yet)

## Tickets

### Ticket 1: Block Parsing

**Summary:** Parse block-level elements.

**Definition of Done:** Paragraphs, headers, lists, and code blocks parsed.

### Ticket 2: Inline Parsing

**Summary:** Parse inline elements within blocks.

**Definition of Done:** Links, emphasis, and code spans parsed.

### Ticket 3: HTML Rendering

**Summary:** Convert AST to HTML output.

**Definition of Done:** Valid HTML generated for all elements.
