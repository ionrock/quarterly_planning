---
id: "test-012"
title: "JSON Schema Validator"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a JSON Schema validator library that supports draft-07 specification. Provides validation, error reporting, and schema compilation for performance.

## Constraints

- Validate 10,000 documents per second
- Full draft-07 compliance

## Implementation Notes

- Written in TypeScript
- Compile schemas for repeated validation
- Detailed error messages with JSON pointers

## Review Notes

(none yet)

## Tickets

### Ticket 1: Schema Parsing

**Summary:** Parse and normalize JSON schemas.

**Definition of Done:** Schemas loaded and $ref resolved.

### Ticket 2: Validation Logic

**Summary:** Implement all validation keywords.

**Definition of Done:** All draft-07 keywords supported.

### Ticket 3: Error Reporting

**Summary:** Generate detailed validation errors.

**Definition of Done:** Errors include path and context.
