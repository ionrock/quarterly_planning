---
id: "test-011"
title: "Static Site Generator"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a static site generator with Markdown support, templating, and incremental builds. Generates HTML from content files with frontmatter metadata.

## Constraints

- Build 1000 pages in under 10 seconds
- Hot reload in development mode

## Implementation Notes

- Rust for performance
- Liquid templates
- CommonMark for Markdown

## Review Notes

(none yet)

## Tickets

### Ticket 1: Content Processing

**Summary:** Parse Markdown with frontmatter and render to HTML.

**Definition of Done:** All Markdown features render correctly.

### Ticket 2: Template Engine

**Summary:** Liquid templates with layouts and partials.

**Definition of Done:** Templates render with variables and includes.

### Ticket 3: Build System

**Summary:** Incremental builds with dependency tracking.

**Definition of Done:** Only changed files are rebuilt.
