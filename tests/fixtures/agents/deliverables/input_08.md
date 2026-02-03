---
id: "test-008"
title: "Log Aggregation Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a log aggregation service that collects, indexes, and searches application logs. Supports structured logging, full-text search, and alerting.

## Constraints

- Ingest 100,000 logs per second
- Search latency under 500ms

## Implementation Notes

- Rust for log ingestion
- ClickHouse for storage
- Vector for log shipping

## Review Notes

(none yet)

## Tickets

### Ticket 1: Log Ingestion

**Summary:** HTTP and TCP endpoints for log ingestion.

**Definition of Done:** Logs received and stored reliably.

### Ticket 2: Search API

**Summary:** Full-text search with filtering.

**Definition of Done:** Logs searchable by any field.

### Ticket 3: Alerting

**Summary:** Trigger alerts based on log patterns.

**Definition of Done:** Alerts fire when conditions match.
