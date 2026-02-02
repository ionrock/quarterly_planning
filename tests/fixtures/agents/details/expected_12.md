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

### Technology Stack
- **Language:** TypeScript 5.x
- **Build:** esbuild for fast bundling + tree-shaking
- **Testing:** Vitest with JSON Schema Test Suite

### Core Interfaces
```typescript
interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
}

interface ValidationError {
  message: string;
  instancePath: string;      // JSON Pointer to failing location
  schemaPath: string;        // JSON Pointer to schema keyword
  keyword: string;           // e.g., "type", "required"
  params: Record<string, unknown>;
}

interface Schema {
  $id?: string;
  $ref?: string;
  $defs?: Record<string, Schema>;
  type?: string | string[];
  properties?: Record<string, Schema>;
  required?: string[];
  items?: Schema | Schema[];
  // ... all draft 2020-12 keywords
}

type ValidateFunction = (data: unknown) => ValidationResult;
```

### Validator Class
```typescript
class JSONSchemaValidator {
  private schemas: Map<string, Schema> = new Map();
  private compiled: Map<string, ValidateFunction> = new Map();

  addSchema(schema: Schema, id?: string): void {
    const schemaId = id ?? schema.$id ?? `anonymous:${this.schemas.size}`;
    this.schemas.set(schemaId, schema);
  }

  compile(schema: Schema): ValidateFunction {
    const context: CompilationContext = {
      root: schema,
      schemas: this.schemas,
      path: [],
    };
    return this.compileSchema(schema, context);
  }

  validate(schema: Schema, data: unknown): ValidationResult {
    const validate = this.compile(schema);
    return validate(data);
  }
}
```

### Keyword Validators
```typescript
const keywordValidators: Record<string, KeywordValidator> = {
  type: (schema, data, ctx) => {
    const types = Array.isArray(schema.type) ? schema.type : [schema.type];
    const actualType = getType(data);
    if (!types.includes(actualType)) {
      return error(ctx, 'type', `Expected ${types.join('|')}, got ${actualType}`, { expected: types, actual: actualType });
    }
    return valid();
  },

  properties: (schema, data, ctx) => {
    if (typeof data !== 'object' || data === null) return valid();
    const errors: ValidationError[] = [];
    for (const [key, propSchema] of Object.entries(schema.properties ?? {})) {
      if (key in data) {
        const result = validateSchema(propSchema, data[key], pushPath(ctx, key));
        errors.push(...result.errors);
      }
    }
    return { valid: errors.length === 0, errors };
  },

  required: (schema, data, ctx) => {
    if (typeof data !== 'object' || data === null) return valid();
    const missing = (schema.required ?? []).filter(key => !(key in data));
    if (missing.length > 0) {
      return error(ctx, 'required', `Missing required properties: ${missing.join(', ')}`, { missing });
    }
    return valid();
  },

  $ref: (schema, data, ctx) => {
    const refSchema = resolveRef(schema.$ref!, ctx);
    return validateSchema(refSchema, data, ctx);
  },
};
```

### Reference Resolution
```typescript
function resolveRef(ref: string, ctx: CompilationContext): Schema {
  if (ref.startsWith('#')) {
    // Local reference
    return resolvePointer(ctx.root, ref.slice(1));
  }

  const [baseUri, fragment] = ref.split('#');
  const schema = ctx.schemas.get(baseUri);
  if (!schema) {
    throw new Error(`Unknown schema: ${baseUri}`);
  }

  return fragment ? resolvePointer(schema, fragment) : schema;
}

function resolvePointer(obj: unknown, pointer: string): Schema {
  const parts = pointer.split('/').slice(1).map(decodeJsonPointer);
  let current = obj;
  for (const part of parts) {
    current = (current as Record<string, unknown>)[part];
  }
  return current as Schema;
}
```

### Format Validators
```typescript
const formatValidators: Record<string, (value: string) => boolean> = {
  'date-time': (v) => !isNaN(Date.parse(v)) && /^\d{4}-\d{2}-\d{2}T/.test(v),
  'date': (v) => /^\d{4}-\d{2}-\d{2}$/.test(v),
  'email': (v) => /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(v),
  'uri': (v) => { try { new URL(v); return true; } catch { return false; } },
  'uuid': (v) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(v),
};
```

### Compilation for Performance
```typescript
function compileSchema(schema: Schema, ctx: CompilationContext): ValidateFunction {
  // Generate optimized validation function
  const checks: Array<(data: unknown) => ValidationError[]> = [];

  if (schema.type) {
    checks.push(compileTypeCheck(schema.type));
  }
  if (schema.properties) {
    checks.push(compilePropertiesCheck(schema.properties, ctx));
  }
  // ... compile other keywords

  return (data: unknown): ValidationResult => {
    const errors: ValidationError[] = [];
    for (const check of checks) {
      errors.push(...check(data));
    }
    return { valid: errors.length === 0, errors };
  };
}
```

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
