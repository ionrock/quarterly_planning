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

### Technology Stack
- **Language:** Go 1.21+
- **WebSocket:** gorilla/websocket v1.5+
- **Router:** chi or standard net/http
- **Logging:** zerolog for structured logging

### Core Data Structures
```go
type Client struct {
    ID       string
    Conn     *websocket.Conn
    Room     *Room
    Send     chan []byte    // Buffered channel for outbound messages
    mu       sync.Mutex     // Protects Conn writes
}

type Room struct {
    ID      string
    Clients map[string]*Client
    mu      sync.RWMutex
}

type Hub struct {
    Rooms      map[string]*Room
    Register   chan *Client
    Unregister chan *Client
    mu         sync.RWMutex
}
```

### Message Protocol
```go
type MessageType string

const (
    TypeJoin    MessageType = "join"
    TypeLeave   MessageType = "leave"
    TypeMessage MessageType = "message"
    TypeError   MessageType = "error"
    TypeAck     MessageType = "ack"
)

type Message struct {
    Type      MessageType `json:"type"`
    Room      string      `json:"room,omitempty"`
    Content   string      `json:"content,omitempty"`
    Sender    string      `json:"sender,omitempty"`
    Timestamp int64       `json:"timestamp"`
    ID        string      `json:"id,omitempty"`
}
```

### WebSocket Upgrade Handler
```go
var upgrader = websocket.Upgrader{
    ReadBufferSize:  1024,
    WriteBufferSize: 1024,
    CheckOrigin: func(r *http.Request) bool {
        // TODO: Implement proper origin checking
        return true
    },
}

func (h *Hub) HandleWebSocket(w http.ResponseWriter, r *http.Request) {
    conn, err := upgrader.Upgrade(w, r, nil)
    if err != nil {
        log.Error().Err(err).Msg("websocket upgrade failed")
        return
    }

    client := &Client{
        ID:   uuid.New().String(),
        Conn: conn,
        Send: make(chan []byte, 256),
    }

    h.Register <- client

    go client.writePump()
    go client.readPump(h)
}
```

### Client Read/Write Pumps
```go
func (c *Client) readPump(h *Hub) {
    defer func() {
        h.Unregister <- c
        c.Conn.Close()
    }()

    c.Conn.SetReadLimit(maxMessageSize) // 4KB
    c.Conn.SetReadDeadline(time.Now().Add(pongWait)) // 60s
    c.Conn.SetPongHandler(func(string) error {
        c.Conn.SetReadDeadline(time.Now().Add(pongWait))
        return nil
    })

    for {
        _, message, err := c.Conn.ReadMessage()
        if err != nil {
            if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
                log.Error().Err(err).Str("client", c.ID).Msg("read error")
            }
            break
        }
        h.handleMessage(c, message)
    }
}

func (c *Client) writePump() {
    ticker := time.NewTicker(pingPeriod) // 30s
    defer func() {
        ticker.Stop()
        c.Conn.Close()
    }()

    for {
        select {
        case message, ok := <-c.Send:
            c.Conn.SetWriteDeadline(time.Now().Add(writeWait)) // 10s
            if !ok {
                c.Conn.WriteMessage(websocket.CloseMessage, []byte{})
                return
            }
            c.mu.Lock()
            err := c.Conn.WriteMessage(websocket.TextMessage, message)
            c.mu.Unlock()
            if err != nil {
                return
            }
        case <-ticker.C:
            c.Conn.SetWriteDeadline(time.Now().Add(writeWait))
            if err := c.Conn.WriteMessage(websocket.PingMessage, nil); err != nil {
                return
            }
        }
    }
}
```

### Room Broadcasting
```go
func (r *Room) Broadcast(message []byte, exclude *Client) {
    r.mu.RLock()
    defer r.mu.RUnlock()

    for _, client := range r.Clients {
        if client != exclude {
            select {
            case client.Send <- message:
            default:
                // Client buffer full, close connection
                close(client.Send)
                delete(r.Clients, client.ID)
            }
        }
    }
}
```

### Connection Lifecycle Constants
```go
const (
    writeWait      = 10 * time.Second
    pongWait       = 60 * time.Second
    pingPeriod     = (pongWait * 9) / 10
    maxMessageSize = 4096
)
```

### Graceful Shutdown
```go
func (h *Hub) Shutdown(ctx context.Context) error {
    h.mu.Lock()
    defer h.mu.Unlock()

    for _, room := range h.Rooms {
        room.mu.Lock()
        for _, client := range room.Clients {
            client.Conn.WriteControl(
                websocket.CloseMessage,
                websocket.FormatCloseMessage(websocket.CloseGoingAway, "server shutdown"),
                time.Now().Add(time.Second),
            )
            client.Conn.Close()
        }
        room.mu.Unlock()
    }
    return nil
}
```

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
