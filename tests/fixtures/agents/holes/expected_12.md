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

### Identified Weaknesses

1. **Circular reference handling**: $ref can create cycles; need detection and handling strategy.

2. **Remote schema fetching undefined**: How are external $ref URIs resolved? Sync vs async?

3. **No caching strategy**: Compiled schemas should be cached, but how?

4. **50KB bundle size is ambitious**: Draft 2020-12 has many keywords; may need tree-shaking.

5. **No format validation mentioned**: String formats (email, uri, date) are optional but expected.

### Edge Cases

- Deeply nested schemas (stack overflow risk)
- Very large arrays/objects (performance)
- Unicode in property names and strings
- Empty schemas ({} validates everything)
- Boolean schemas (true/false)
- Conflicting keywords (e.g., type + const)
- Recursive schemas ($ref to self)

### Assumptions to Validate

- Is async schema loading required, or preloaded only?
- Which optional format validators should be included?
- Should we support older drafts for compatibility?
- Is schema compilation required, or just interpretation?
- Do we need custom keyword extension support?

### Potential Failures

- Stack overflow on deeply recursive schemas
- Performance degradation on large documents
- Memory leak from unbounded schema cache
- Incorrect handling of numeric precision (JSON numbers)
- Browser/Node.js compatibility issues
- Tree-shaking not working, exceeding bundle size

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
