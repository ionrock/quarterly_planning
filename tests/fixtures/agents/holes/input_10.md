---
id: "test-010"
title: "Linked List Library"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a generic linked list library in Rust with both singly and doubly linked list implementations. Provide common operations like insert, delete, search, and iteration. Focus on memory safety and zero-cost abstractions.

## Constraints

- No unsafe code unless absolutely necessary
- Must be no_std compatible

## Implementation Notes

- Use Box for heap allocation
- Implement Iterator trait for traversal
- Generic over element type with appropriate trait bounds
- Provide both mutable and immutable access patterns

## Review Notes

(none yet)

## Tickets

### Ticket 1: Singly Linked List

**Summary:** Implement singly linked list with basic operations.

**Definition of Done:** Insert, delete, and search operations work correctly.

### Ticket 2: Doubly Linked List

**Summary:** Implement doubly linked list with bidirectional traversal.

**Definition of Done:** Forward and backward iteration works.

### Ticket 3: Iterator Implementation

**Summary:** Implement Iterator trait for both list types.

**Definition of Done:** Lists work with for loops and iterator methods.
