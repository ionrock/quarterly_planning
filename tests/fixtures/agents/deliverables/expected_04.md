---
id: "test-004"
title: "Real-time Collaborative Editor"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a real-time collaborative text editor using CRDTs. Multiple users can edit the same document simultaneously without conflicts. Web-based with offline support.

## Constraints

- Must work with 50+ concurrent editors
- Sync latency under 200ms

## Implementation Notes

- Yjs for CRDT implementation
- WebSocket for real-time sync
- IndexedDB for offline storage

## Review Notes

(none yet)

## Tickets

### Ticket 1: CRDT Document Model

**Summary:** Implement document model using Yjs.

**Definition of Done:** Local edits merge correctly with remote changes.

#### Acceptance Criteria

1. **Document Structure**
   - [ ] Y.Doc created for each document
   - [ ] Y.Text used for main content
   - [ ] Y.Map used for document metadata (title, owner, etc.)
   - [ ] Document ID stored in metadata

2. **Text Operations**
   - [ ] Insert text at position works
   - [ ] Delete range of text works
   - [ ] Replace text (delete + insert) works
   - [ ] Operations generate minimal update messages

3. **Merge Behavior**
   - [ ] Concurrent inserts at same position both appear
   - [ ] Concurrent deletes of same text work correctly
   - [ ] Insert into deleted range preserved
   - [ ] No data loss on any merge

4. **State Management**
   - [ ] Full document state exportable as Uint8Array
   - [ ] State vector trackable for sync
   - [ ] Updates applyable incrementally
   - [ ] Undo manager tracks local changes

#### Demo Script
```javascript
// Create two documents (simulating two clients)
const doc1 = new Y.Doc();
const doc2 = new Y.Doc();

// Connect them
doc1.on('update', update => Y.applyUpdate(doc2, update));
doc2.on('update', update => Y.applyUpdate(doc1, update));

// Concurrent edits
const text1 = doc1.getText('content');
const text2 = doc2.getText('content');

text1.insert(0, 'Hello ');
text2.insert(0, 'World');

// Both documents converge
console.log(text1.toString()); // "Hello World" or "WorldHello " (deterministic)
console.log(text2.toString()); // Same as above
```

#### Test Requirements
- [ ] Unit tests for all text operations
- [ ] Fuzz tests with random concurrent operations
- [ ] Test convergence with 10+ simulated clients
- [ ] Benchmark: 1000 ops/second merge performance

### Ticket 2: WebSocket Sync

**Summary:** Real-time synchronization between clients.

**Definition of Done:** Changes propagate to all connected clients.

#### Acceptance Criteria

1. **Connection Management**
   - [ ] WebSocket connection established on document open
   - [ ] Automatic reconnection with exponential backoff
   - [ ] Connection status indicator (connected/connecting/disconnected)
   - [ ] Heartbeat ping every 30 seconds

2. **Sync Protocol**
   - [ ] Initial sync: client sends state vector, server sends missing updates
   - [ ] Incremental sync: updates broadcast to all clients
   - [ ] Awareness protocol for cursor positions
   - [ ] Binary messages (not JSON) for efficiency

3. **Server Implementation**
   - [ ] Room-based document isolation
   - [ ] In-memory document state with persistence hook
   - [ ] Client join/leave events
   - [ ] Max clients per room configurable (default 100)

4. **Latency**
   - [ ] Local changes apply immediately (optimistic)
   - [ ] Remote changes apply within 200ms
   - [ ] No visible lag during normal typing

#### Demo Script
```bash
# Start sync server
npm run server

# Open document in browser 1
open http://localhost:3000/doc/test-doc

# Open same document in browser 2
open http://localhost:3000/doc/test-doc

# Type in browser 1, see changes in browser 2 within 200ms
# Cursor position of browser 1 visible in browser 2
```

#### Test Requirements
- [ ] Integration test with WebSocket mock
- [ ] Test reconnection after disconnect
- [ ] Test sync with simulated latency (500ms)
- [ ] Load test: 50 clients, continuous typing
- [ ] Measure p99 sync latency

### Ticket 3: Offline Support

**Summary:** Enable editing while disconnected.

**Definition of Done:** Offline changes sync when reconnected.

#### Acceptance Criteria

1. **Local Persistence**
   - [ ] Document state saved to IndexedDB
   - [ ] Saves triggered on every change (debounced 100ms)
   - [ ] Document loads from IndexedDB on page load
   - [ ] Storage quota handled gracefully

2. **Offline Detection**
   - [ ] Navigator.onLine checked
   - [ ] WebSocket disconnect detected
   - [ ] UI indicates offline mode
   - [ ] Edits continue working offline

3. **Sync on Reconnect**
   - [ ] Pending updates queued locally
   - [ ] Queue sent on reconnection
   - [ ] Server updates received and merged
   - [ ] Conflict-free merge (CRDT guarantees this)

4. **Persistence Recovery**
   - [ ] App works after browser restart
   - [ ] Multiple tabs sync via BroadcastChannel
   - [ ] Corrupted IndexedDB handled (reset option)

#### Demo Script
```bash
# Open document
open http://localhost:3000/doc/test-doc

# Type some text
# Disconnect network (airplane mode or devtools offline)

# Continue typing - changes saved locally
# "Offline" indicator appears

# Reconnect network
# Changes sync to server within 5 seconds
# "Online" indicator appears

# Refresh page - all changes still present
```

#### Test Requirements
- [ ] Test IndexedDB persistence
- [ ] Test offline edit and reconnect
- [ ] Test with 1000 offline changes
- [ ] Test corrupted storage recovery
- [ ] Test multi-tab synchronization
