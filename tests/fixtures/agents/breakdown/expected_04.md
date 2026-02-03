---
id: "test-004"
title: "WebSocket Chat Server"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a real-time chat server using WebSockets. Supports multiple chat rooms, user presence, and message history. Built with Node.js and Socket.io.

## Constraints

- Support 10,000 concurrent connections
- Message delivery under 100ms

## Implementation Notes

- Use Socket.io for WebSocket handling
- Redis pub/sub for horizontal scaling
- PostgreSQL for message persistence

## Review Notes

(none yet)

## Tickets

### Ticket 1: WebSocket Server

**Summary:** Set up Socket.io server with connection handling.

**Definition of Done:** Clients can connect and receive events.

#### Steps

1. **Install Socket.io dependencies**
   - Run `npm install socket.io @socket.io/redis-adapter`
   - Verify: packages added to package.json

2. **Create Socket.io server module**
   - Create src/socket/server.ts
   - Initialize Socket.io server attached to HTTP server
   - Verify: module exports io instance

3. **Configure CORS for WebSocket**
   - Set cors option with allowed origins from env
   - Allow credentials for authenticated connections
   - Verify: browser can connect from allowed origin

4. **Implement connection event handler**
   - Listen for 'connection' event on io
   - Log socket.id and connection metadata
   - Verify: connection logged when client connects

5. **Implement disconnect handler**
   - Listen for 'disconnect' event on each socket
   - Log disconnect reason
   - Verify: disconnect logged when client leaves

6. **Add authentication middleware**
   - Create src/socket/middleware/auth.ts
   - Verify JWT token from handshake auth
   - Attach user data to socket object
   - Verify: unauthenticated connections rejected

7. **Set up Redis adapter for scaling**
   - Create Redis pub/sub clients
   - Configure socket.io-redis adapter
   - Verify: events propagate between server instances

8. **Add heartbeat monitoring**
   - Configure pingTimeout and pingInterval
   - Log stale connections
   - Verify: dead connections detected and cleaned up

### Ticket 2: Chat Rooms

**Summary:** Implement room join/leave and messaging.

**Definition of Done:** Users can join rooms and send messages.

#### Steps

1. **Create room management module**
   - Create src/socket/rooms.ts
   - Define Room interface with id, name, members
   - Verify: module exports correctly

2. **Implement join room handler**
   - Listen for 'room:join' event with roomId payload
   - Call socket.join(roomId)
   - Broadcast 'room:user_joined' to room
   - Verify: user added to room, others notified

3. **Implement leave room handler**
   - Listen for 'room:leave' event with roomId payload
   - Call socket.leave(roomId)
   - Broadcast 'room:user_left' to room
   - Verify: user removed from room, others notified

4. **Handle disconnect room cleanup**
   - On disconnect, get socket.rooms
   - Broadcast 'room:user_left' to each room
   - Verify: disconnected users removed from all rooms

5. **Implement send message handler**
   - Listen for 'message:send' event with { roomId, content }
   - Validate user is in room
   - Broadcast 'message:new' to room with message data
   - Verify: message received by all room members

6. **Add message validation**
   - Check content length (max 10000 chars)
   - Sanitize HTML/script content
   - Verify: oversized and malicious messages rejected

7. **Implement typing indicator**
   - Listen for 'typing:start' and 'typing:stop' events
   - Broadcast to room (except sender)
   - Verify: typing indicators appear for other users

8. **Track room member list**
   - Maintain in-memory or Redis set of room members
   - Emit 'room:members' with current list on join
   - Verify: new joiners receive member list

9. **Implement user presence**
   - Create src/socket/presence.ts
   - Track online/offline/away status per user
   - Broadcast status changes to relevant rooms
   - Verify: presence updates visible to room members

### Ticket 3: Message Persistence

**Summary:** Store and retrieve message history.

**Definition of Done:** Messages are saved and history is loadable.

#### Steps

1. **Create messages database table**
   - Create migration for messages table
   - Columns: id, room_id, user_id, content, created_at
   - Add index on (room_id, created_at)
   - Verify: migration runs successfully

2. **Create Message model**
   - Create src/models/message.ts
   - Define Message interface matching table schema
   - Verify: interface matches database columns

3. **Create message repository**
   - Create src/repositories/messageRepository.ts
   - Implement create(message): Promise<Message>
   - Verify: messages insert correctly

4. **Save messages on send**
   - In 'message:send' handler, call repository.create()
   - Include user_id from authenticated socket
   - Don't await - persist asynchronously
   - Verify: messages appear in database after sending

5. **Implement history fetch handler**
   - Listen for 'message:history' event with { roomId, before?, limit? }
   - Query messages with cursor-based pagination
   - Emit 'message:history' with messages array
   - Verify: client receives paginated history

6. **Add history query to repository**
   - Implement getByRoom(roomId, { before, limit }): Promise<Message[]>
   - Order by created_at DESC, limit to 50 default
   - Verify: returns correct messages in order

7. **Send initial history on room join**
   - After successful join, fetch last 50 messages
   - Emit 'message:history' to joining user only
   - Verify: new room members see recent messages

8. **Add message read receipts table**
   - Create migration for read_receipts table
   - Columns: user_id, room_id, last_read_at, last_read_message_id
   - Verify: migration runs successfully

9. **Track read position**
   - Listen for 'message:read' event with { roomId, messageId }
   - Update read receipt in database
   - Broadcast 'message:read' to room for read receipt UI
   - Verify: read positions tracked correctly

10. **Implement unread count**
    - Create getUnreadCount(userId, roomId): Promise<number>
    - Count messages after last_read_message_id
    - Include in room list response
    - Verify: unread counts accurate
