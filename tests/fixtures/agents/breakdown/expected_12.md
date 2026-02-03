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

#### Steps

1. **Create TypeScript project**
   - Run `npm init -y && npm install -D typescript @types/node`
   - Create tsconfig.json with strict mode
   - Verify: `npx tsc` compiles

2. **Define Schema type**
   - Create src/types.ts
   - Define JSONSchema interface with all keywords
   - Verify: type compiles

3. **Create schema loader**
   - Create src/loader.ts
   - Load schema from object or file path
   - Verify: schemas load correctly

4. **Implement $id extraction**
   - Extract base URI from $id keyword
   - Store in schema registry
   - Verify: $id parsed correctly

5. **Build schema registry**
   - Create src/registry.ts
   - Map $id to schema objects
   - Support adding/getting schemas
   - Verify: schemas retrievable by $id

6. **Implement $ref resolution**
   - Create src/ref.ts
   - Parse JSON Pointer from $ref
   - Resolve against base URI
   - Verify: local refs resolve

7. **Handle remote $ref**
   - Fetch remote schemas via HTTP
   - Cache fetched schemas
   - Verify: remote refs resolve

8. **Handle recursive $ref**
   - Detect circular references
   - Use lazy resolution
   - Verify: recursive schemas work

9. **Normalize schema structure**
   - Expand shorthand (type as array)
   - Set defaults for optional keywords
   - Verify: normalized schemas consistent

### Ticket 2: Validation Logic

**Summary:** Implement all validation keywords.

**Definition of Done:** All draft-07 keywords supported.

#### Steps

1. **Create validator interface**
   - Create src/validator.ts
   - Define validate(schema, data): ValidationResult
   - Verify: interface compiles

2. **Implement type keyword**
   - Validate against string, number, integer, boolean, array, object, null
   - Support type as array (any of)
   - Verify: type validation works

3. **Implement string keywords**
   - minLength, maxLength
   - pattern (regex)
   - format (email, uri, date-time, etc.)
   - Verify: string validation works

4. **Implement number keywords**
   - minimum, maximum
   - exclusiveMinimum, exclusiveMaximum
   - multipleOf
   - Verify: number validation works

5. **Implement array keywords**
   - items (single schema or array)
   - additionalItems
   - minItems, maxItems
   - uniqueItems
   - contains
   - Verify: array validation works

6. **Implement object keywords**
   - properties
   - patternProperties
   - additionalProperties
   - required
   - propertyNames
   - minProperties, maxProperties
   - Verify: object validation works

7. **Implement conditional keywords**
   - if/then/else
   - Verify: conditionals work

8. **Implement composition keywords**
   - allOf (all must validate)
   - anyOf (at least one)
   - oneOf (exactly one)
   - not (must not validate)
   - Verify: composition works

9. **Implement const and enum**
   - const (exact match)
   - enum (one of values)
   - Verify: value constraints work

10. **Implement definitions**
    - Support definitions/$defs keyword
    - Resolve refs to definitions
    - Verify: definitions work

11. **Create compiled validator**
    - Pre-compute validation functions
    - Cache compiled validators
    - Verify: compilation improves performance

### Ticket 3: Error Reporting

**Summary:** Generate detailed validation errors.

**Definition of Done:** Errors include path and context.

#### Steps

1. **Define ValidationError type**
   - Create src/errors.ts
   - Include: keyword, path, message, schemaPath, data
   - Verify: type compiles

2. **Track data path during validation**
   - Build JSON Pointer as validating
   - Pass path to nested validators
   - Verify: paths track correctly

3. **Track schema path**
   - Record schema location for each check
   - Include in error for debugging
   - Verify: schema paths correct

4. **Generate meaningful messages**
   - Create message templates per keyword
   - Include actual vs expected values
   - Verify: messages are helpful

5. **Collect all errors (not just first)**
   - Configure allErrors option
   - Continue validation after error
   - Verify: all errors collected

6. **Format errors as JSON**
   - Implement toJSON() on errors
   - Include all context
   - Verify: JSON errors parseable

7. **Create human-readable format**
   - Implement toString() on errors
   - Format path and message nicely
   - Verify: errors readable

8. **Add error codes**
   - Assign unique code per error type
   - Enable programmatic handling
   - Verify: codes consistent

9. **Support custom error messages**
   - Allow errorMessage keyword in schema
   - Override default messages
   - Verify: custom messages used

10. **Create validation result type**
    - Include valid boolean
    - Include errors array
    - Include validated data (with defaults applied)
    - Verify: result type complete

11. **Write comprehensive test suite**
    - Use official JSON Schema test suite
    - Test all keywords
    - Test error messages
    - Verify: passes test suite

12. **Benchmark performance**
    - Create benchmark script
    - Measure validations per second
    - Verify: meets 10,000/s target
