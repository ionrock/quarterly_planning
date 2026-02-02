---
id: "test-012"
title: "JSON Schema Validator"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a JSON Schema validator library that validates JSON documents against JSON Schema draft 2020-12. Supports all standard keywords and provides detailed error messages. Written in TypeScript for Node.js and browser use.

## Constraints

- Must pass the official JSON Schema test suite
- Bundle size under 50KB minified

## Implementation Notes

- Parse schema and compile to validation function
- Support $ref for schema references
- Implement all validation keywords
- Return structured error objects with JSON Pointer paths

## Review Notes

(none yet)

## Tickets

### Ticket 1: Schema Parser

**Summary:** Parse JSON Schema documents and resolve references.

**Definition of Done:** Schemas with $ref are correctly dereferenced.

### Ticket 2: Validation Keywords

**Summary:** Implement all validation keywords (type, properties, etc).

**Definition of Done:** All keywords validate correctly.

### Ticket 3: Error Reporting

**Summary:** Generate detailed error messages with paths.

**Definition of Done:** Errors include JSON Pointer to failing location.
