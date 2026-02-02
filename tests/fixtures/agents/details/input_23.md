---
id: "test-023"
title: "API Documentation Generator"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a tool that generates API documentation from OpenAPI/Swagger specs. Produces a static HTML site with interactive examples, search, and dark mode. Written in TypeScript.

## Constraints

- Support OpenAPI 3.0 and 3.1
- Generated site must work offline

## Implementation Notes

- Parse OpenAPI spec with swagger-parser
- Generate static HTML with Vite
- Include try-it-out functionality
- Support multiple specs for multi-service docs

## Review Notes

(none yet)

## Tickets

### Ticket 1: Spec Parser

**Summary:** Parse and validate OpenAPI specifications.

**Definition of Done:** Valid specs are parsed, invalid ones error clearly.

### Ticket 2: HTML Generator

**Summary:** Generate static HTML documentation.

**Definition of Done:** Documentation renders correctly in browsers.

### Ticket 3: Interactive Features

**Summary:** Add search and try-it-out functionality.

**Definition of Done:** Users can search and make test requests.
