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

### Identified Weaknesses

1. **No message persistence**: In-memory only means all chat history is lost on restart.

2. **No user authentication**: How are users identified? Anyone can impersonate anyone.

3. **Single server limitation**: In-memory storage won't work with multiple server instances or horizontal scaling.

4. **No message ordering guarantees**: Concurrent messages might arrive out of order.

5. **Missing connection health checks**: No ping/pong or heartbeat mechanism to detect dead connections.

### Edge Cases

- What happens when a user's connection drops mid-message?
- How are duplicate connections from the same user handled?
- What if a room has 0 users? Is it cleaned up?
- Maximum message size limits?
- What about special characters, unicode, or emoji in messages?
- How are very large rooms (1000+ users) handled for broadcasts?

### Assumptions to Validate

- Is message history needed, or is ephemeral chat acceptable?
- Do we need private/direct messages or only room-based?
- Is there an existing user system to integrate with?
- What's the expected message rate per user?
- Are there content moderation requirements?

### Potential Failures

- Memory exhaustion with many connections/rooms
- Slow client blocking broadcasts (need write timeouts)
- Connection leak if cleanup fails on disconnect
- Thundering herd on server restart (all clients reconnect)
- No graceful shutdown mechanism

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
