---
id: "test-025"
title: "Event Sourcing Library"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create an event sourcing library for Python applications. Provides aggregate base class, event store interface, and snapshot support. Implements optimistic concurrency control. Backends for PostgreSQL and in-memory (testing).

## Constraints

- Support aggregates with millions of events via snapshots
- Event replay under 100ms for typical aggregates

## Implementation Notes

### Technology Stack
- **Language:** Python 3.11+
- **Database:** PostgreSQL with asyncpg
- **Serialization:** msgpack for events (fast, compact)
- **Typing:** Full type hints with generics

### Core Abstractions
```python
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from datetime import datetime
from typing import Generic, TypeVar, List, Optional
from uuid import UUID, uuid4
import msgpack

@dataclass
class Event:
    event_id: UUID = field(default_factory=uuid4)
    event_type: str = ""
    timestamp: datetime = field(default_factory=datetime.utcnow)
    data: dict = field(default_factory=dict)
    metadata: dict = field(default_factory=dict)

T = TypeVar('T', bound='Aggregate')

class Aggregate(ABC, Generic[T]):
    id: UUID
    version: int
    _pending_events: List[Event]

    def __init__(self, id: UUID):
        self.id = id
        self.version = 0
        self._pending_events = []

    @abstractmethod
    def apply(self, event: Event) -> None:
        """Apply an event to update aggregate state."""
        pass

    def _record(self, event: Event) -> None:
        """Record a new event and apply it."""
        self._pending_events.append(event)
        self.apply(event)

    def clear_pending(self) -> List[Event]:
        events = self._pending_events[:]
        self._pending_events = []
        return events

    @classmethod
    def from_events(cls, id: UUID, events: List[Event]) -> T:
        instance = cls(id)
        for event in events:
            instance.apply(event)
            instance.version += 1
        return instance
```

### Event Store Interface
```python
class EventStore(ABC):
    @abstractmethod
    async def append(
        self,
        stream_id: str,
        events: List[Event],
        expected_version: int
    ) -> int:
        """Append events to stream. Returns new version.
        Raises ConcurrencyError if expected_version doesn't match."""
        pass

    @abstractmethod
    async def read(
        self,
        stream_id: str,
        from_version: int = 0,
        to_version: Optional[int] = None
    ) -> List[Event]:
        """Read events from stream within version range."""
        pass

    @abstractmethod
    async def read_all(
        self,
        from_position: int = 0,
        batch_size: int = 1000
    ) -> AsyncIterator[Event]:
        """Read all events across all streams."""
        pass
```

### PostgreSQL Event Store
```python
class PostgresEventStore(EventStore):
    def __init__(self, pool: asyncpg.Pool):
        self.pool = pool

    async def append(
        self,
        stream_id: str,
        events: List[Event],
        expected_version: int
    ) -> int:
        async with self.pool.acquire() as conn:
            async with conn.transaction():
                # Check current version
                row = await conn.fetchrow(
                    "SELECT version FROM streams WHERE id = $1 FOR UPDATE",
                    stream_id
                )
                current_version = row['version'] if row else 0

                if current_version != expected_version:
                    raise ConcurrencyError(
                        f"Expected version {expected_version}, got {current_version}"
                    )

                # Insert events
                new_version = current_version
                for event in events:
                    new_version += 1
                    await conn.execute("""
                        INSERT INTO events (stream_id, version, event_id, event_type, data, metadata, timestamp)
                        VALUES ($1, $2, $3, $4, $5, $6, $7)
                    """,
                        stream_id, new_version, event.event_id,
                        event.event_type, msgpack.packb(event.data),
                        msgpack.packb(event.metadata), event.timestamp
                    )

                # Update stream version
                await conn.execute("""
                    INSERT INTO streams (id, version) VALUES ($1, $2)
                    ON CONFLICT (id) DO UPDATE SET version = $2
                """, stream_id, new_version)

                return new_version

    async def read(
        self,
        stream_id: str,
        from_version: int = 0,
        to_version: Optional[int] = None
    ) -> List[Event]:
        query = """
            SELECT event_id, event_type, data, metadata, timestamp
            FROM events
            WHERE stream_id = $1 AND version > $2
        """
        args = [stream_id, from_version]

        if to_version is not None:
            query += " AND version <= $3"
            args.append(to_version)

        query += " ORDER BY version"

        rows = await self.pool.fetch(query, *args)
        return [
            Event(
                event_id=row['event_id'],
                event_type=row['event_type'],
                data=msgpack.unpackb(row['data']),
                metadata=msgpack.unpackb(row['metadata']),
                timestamp=row['timestamp'],
            )
            for row in rows
        ]
```

### Snapshot Support
```python
@dataclass
class Snapshot:
    aggregate_id: UUID
    version: int
    state: bytes
    created_at: datetime = field(default_factory=datetime.utcnow)

class SnapshotStore(ABC):
    @abstractmethod
    async def save(self, snapshot: Snapshot) -> None:
        pass

    @abstractmethod
    async def load(self, aggregate_id: UUID) -> Optional[Snapshot]:
        pass

class Repository(Generic[T]):
    def __init__(
        self,
        event_store: EventStore,
        aggregate_class: type[T],
        snapshot_store: Optional[SnapshotStore] = None,
        snapshot_frequency: int = 100
    ):
        self.event_store = event_store
        self.aggregate_class = aggregate_class
        self.snapshot_store = snapshot_store
        self.snapshot_frequency = snapshot_frequency

    async def load(self, id: UUID) -> T:
        stream_id = f"{self.aggregate_class.__name__}-{id}"

        # Try to load from snapshot first
        from_version = 0
        aggregate = None

        if self.snapshot_store:
            snapshot = await self.snapshot_store.load(id)
            if snapshot:
                aggregate = pickle.loads(snapshot.state)
                from_version = snapshot.version

        # Load remaining events
        events = await self.event_store.read(stream_id, from_version)

        if aggregate is None:
            aggregate = self.aggregate_class.from_events(id, events)
        else:
            for event in events:
                aggregate.apply(event)
                aggregate.version += 1

        return aggregate

    async def save(self, aggregate: T) -> None:
        stream_id = f"{type(aggregate).__name__}-{aggregate.id}"
        events = aggregate.clear_pending()

        if not events:
            return

        new_version = await self.event_store.append(
            stream_id, events, aggregate.version
        )

        aggregate.version = new_version

        # Create snapshot if needed
        if self.snapshot_store and new_version % self.snapshot_frequency == 0:
            snapshot = Snapshot(
                aggregate_id=aggregate.id,
                version=new_version,
                state=pickle.dumps(aggregate),
            )
            await self.snapshot_store.save(snapshot)
```

### Example Aggregate
```python
@dataclass
class OrderCreated(Event):
    event_type: str = "OrderCreated"
    customer_id: UUID
    items: List[dict]

@dataclass
class OrderShipped(Event):
    event_type: str = "OrderShipped"
    tracking_number: str

class Order(Aggregate['Order']):
    customer_id: Optional[UUID] = None
    items: List[dict] = field(default_factory=list)
    status: str = "pending"
    tracking_number: Optional[str] = None

    def create(self, customer_id: UUID, items: List[dict]) -> None:
        self._record(OrderCreated(customer_id=customer_id, items=items))

    def ship(self, tracking_number: str) -> None:
        if self.status != "pending":
            raise InvalidOperation("Order already shipped")
        self._record(OrderShipped(tracking_number=tracking_number))

    def apply(self, event: Event) -> None:
        if isinstance(event, OrderCreated):
            self.customer_id = event.customer_id
            self.items = event.items
        elif isinstance(event, OrderShipped):
            self.status = "shipped"
            self.tracking_number = event.tracking_number
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: Event Store

**Summary:** Implement event store with PostgreSQL backend.

**Definition of Done:** Events are stored and retrieved correctly.

### Ticket 2: Aggregate Framework

**Summary:** Create base class for event-sourced aggregates.

**Definition of Done:** Aggregates can apply events and track changes.

### Ticket 3: Snapshot Support

**Summary:** Implement snapshotting for fast aggregate loading.

**Definition of Done:** Aggregates load from snapshots when available.
