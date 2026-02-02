---
id: "test-004"
title: "WebSocket Chat Server"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a real-time chat server using WebSockets. Users can join chat rooms, send messages, and see messages from other users instantly. Built with Go and the gorilla/websocket library.

## Constraints

- Support at least 1000 concurrent connections
- Messages should be delivered within 100ms

## Implementation Notes

- Use gorilla/websocket for WebSocket handling
- In-memory storage for active connections and rooms
- JSON message format for client-server communication
- Broadcast messages to all users in a room

## Review Notes

(none yet)

## Tickets

### Ticket 1: WebSocket Server

**Summary:** Set up Go HTTP server with WebSocket upgrade endpoint.

**Definition of Done:** Clients can establish WebSocket connections.

### Ticket 2: Room Management

**Summary:** Implement join/leave room functionality.

**Definition of Done:** Users can join rooms and are tracked correctly.

### Ticket 3: Message Broadcasting

**Summary:** Broadcast messages to all users in a room.

**Definition of Done:** Messages sent by one user appear for all room members.
