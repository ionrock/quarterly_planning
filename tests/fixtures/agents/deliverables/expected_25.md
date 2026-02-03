---
id: "test-025"
title: "Event Sourcing Library"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create an event sourcing library for Python applications. Provides aggregate base class, event store interface, and snapshot support. Implements optimistic concurrency control.

## Constraints

- Support aggregates with millions of events via snapshots
- Event replay under 100ms for typical aggregates

## Implementation Notes

- Written in Python
- PostgreSQL event store
- msgpack for serialization

## Review Notes

(none yet)

## Tickets

### Ticket 1: Event Store

**Summary:** Implement event persistence and retrieval.

**Definition of Done:** Events stored and retrieved correctly.

#### Acceptance Criteria

1. **Event Schema**
   - [ ] Event has: stream_id, version, event_type, data, metadata, timestamp
   - [ ] stream_id identifies the aggregate
   - [ ] version auto-increments per stream
   - [ ] data contains event payload (JSON/msgpack)
   - [ ] metadata contains correlation_id, causation_id, user_id

2. **Append Events**
   - [ ] `append(stream_id, events, expected_version)` persists events
   - [ ] Optimistic concurrency: fails if version mismatch
   - [ ] Returns new version number
   - [ ] Atomic: all events or none

3. **Read Events**
   - [ ] `read(stream_id, from_version, to_version)` retrieves events
   - [ ] `read_all(from_position, limit)` retrieves across streams
   - [ ] Ordered by version (per stream) or position (global)
   - [ ] Streaming for large result sets

4. **Database Schema**
   - [ ] Table: events (stream_id, version, event_type, data, metadata, timestamp, position)
   - [ ] Primary key: (stream_id, version)
   - [ ] Unique index: position (global ordering)
   - [ ] Index: event_type for projections

5. **Concurrency Control**
   - [ ] Expected version -1: stream must not exist
   - [ ] Expected version N: stream must be at version N
   - [ ] Expected version ANY: no check
   - [ ] Clear error on conflict

#### Demo Script
```python
from eventsourcing import PostgresEventStore

store = PostgresEventStore("postgresql://localhost/events")

# Append events to new stream
events = [
    {"type": "AccountOpened", "data": {"owner": "Alice", "initial_balance": 100}},
    {"type": "MoneyDeposited", "data": {"amount": 50}}
]
version = await store.append("account-123", events, expected_version=-1)
# version: 2

# Append more events
events = [{"type": "MoneyWithdrawn", "data": {"amount": 30}}]
version = await store.append("account-123", events, expected_version=2)
# version: 3

# Optimistic concurrency failure
try:
    await store.append("account-123", events, expected_version=1)
except ConcurrencyError as e:
    print(e)  # Expected version 1, actual 3

# Read events
events = await store.read("account-123")
# [AccountOpened, MoneyDeposited, MoneyWithdrawn]

# Read from specific version
events = await store.read("account-123", from_version=2)
# [MoneyDeposited, MoneyWithdrawn]

# Read all events (for projections)
async for event in store.read_all(from_position=0, limit=1000):
    project_event(event)
```

#### Test Requirements
- [ ] Test append and read
- [ ] Test optimistic concurrency
- [ ] Test concurrent appends (race condition)
- [ ] Test global ordering
- [ ] Benchmark: 10,000 events/second append
- [ ] Test large stream read (100,000 events)

### Ticket 2: Aggregate Framework

**Summary:** Create base class for aggregates.

**Definition of Done:** Aggregates apply events and track changes.

#### Acceptance Criteria

1. **Aggregate Base Class**
   - [ ] `id` property for aggregate identifier
   - [ ] `version` property tracks current version
   - [ ] `_apply(event)` method updates state
   - [ ] `_record(event)` method records new event

2. **Event Application**
   - [ ] `apply_{event_type}` methods for each event type
   - [ ] Automatic dispatch by event type
   - [ ] State updated from event data
   - [ ] No side effects during apply

3. **Change Tracking**
   - [ ] `_pending_events` list of uncommitted events
   - [ ] `commit()` returns and clears pending events
   - [ ] `has_changes` property

4. **Repository**
   - [ ] `load(aggregate_id)` reconstructs from events
   - [ ] `save(aggregate)` persists pending events
   - [ ] Uses event store for persistence
   - [ ] Handles concurrency errors

5. **Rehydration**
   - [ ] Create empty aggregate
   - [ ] Apply events in order
   - [ ] Set version to last event version
   - [ ] Return fully reconstructed aggregate

#### Demo Script
```python
from eventsourcing import Aggregate, Repository

class BankAccount(Aggregate):
    def __init__(self, id: str):
        super().__init__(id)
        self.balance = 0
        self.owner = None

    # Commands
    def open(self, owner: str, initial_balance: int):
        self._record("AccountOpened", {"owner": owner, "balance": initial_balance})

    def deposit(self, amount: int):
        self._record("MoneyDeposited", {"amount": amount})

    def withdraw(self, amount: int):
        if amount > self.balance:
            raise ValueError("Insufficient funds")
        self._record("MoneyWithdrawn", {"amount": amount})

    # Event handlers
    def apply_AccountOpened(self, event):
        self.owner = event.data["owner"]
        self.balance = event.data["balance"]

    def apply_MoneyDeposited(self, event):
        self.balance += event.data["amount"]

    def apply_MoneyWithdrawn(self, event):
        self.balance -= event.data["amount"]

# Usage
repo = Repository(BankAccount, event_store)

# Create new account
account = BankAccount("acc-123")
account.open("Alice", 100)
account.deposit(50)
await repo.save(account)

# Load and modify
account = await repo.load("acc-123")
print(account.balance)  # 150
account.withdraw(30)
await repo.save(account)

# Reload
account = await repo.load("acc-123")
print(account.balance)  # 120
```

#### Test Requirements
- [ ] Test aggregate creation
- [ ] Test event recording and application
- [ ] Test repository load/save
- [ ] Test business logic enforcement
- [ ] Test concurrency conflict handling
- [ ] Test with multiple event types

### Ticket 3: Snapshot Support

**Summary:** Add snapshotting for performance.

**Definition of Done:** Aggregates load from snapshots.

#### Acceptance Criteria

1. **Snapshot Schema**
   - [ ] Snapshot has: stream_id, version, data, timestamp
   - [ ] data contains serialized aggregate state
   - [ ] version matches last applied event version

2. **Snapshot Store**
   - [ ] `save_snapshot(aggregate)` persists snapshot
   - [ ] `load_snapshot(stream_id)` retrieves latest
   - [ ] Configurable snapshot frequency
   - [ ] Multiple snapshots retained (configurable)

3. **Snapshot Loading**
   - [ ] Load latest snapshot first
   - [ ] Apply events after snapshot version
   - [ ] Fall back to full replay if no snapshot
   - [ ] Transparent to aggregate code

4. **Automatic Snapshotting**
   - [ ] Snapshot every N events (e.g., 100)
   - [ ] Snapshot on repository save
   - [ ] Background snapshotting option

5. **Snapshot Cleanup**
   - [ ] Retain last N snapshots
   - [ ] Delete old snapshots
   - [ ] Vacuum script for maintenance

#### Demo Script
```python
from eventsourcing import Repository, SnapshotStore

snapshot_store = PostgresSnapshotStore("postgresql://localhost/events")

repo = Repository(
    BankAccount,
    event_store,
    snapshot_store=snapshot_store,
    snapshot_every=100  # Snapshot every 100 events
)

# Account with many transactions
account = await repo.load("acc-123")  # Has 10,000 events

# With snapshot: loads snapshot + ~50 events = fast
# Without snapshot: loads 10,000 events = slow

# Manual snapshot
await repo.save_snapshot(account)

# Snapshot is taken automatically after 100 new events
for i in range(100):
    account.deposit(1)
await repo.save(account)  # Saves events + creates snapshot

# Verify snapshot usage
account = await repo.load("acc-123")
print(account._loaded_from_snapshot)  # True
print(account._events_after_snapshot)  # ~0
```

```bash
# Performance comparison
# Without snapshots (10,000 events):
time python -c "repo.load('acc-with-10k-events')"
# real: 2.5s

# With snapshot:
time python -c "repo.load('acc-with-10k-events')"
# real: 0.05s (50x faster)

# Cleanup old snapshots
python -m eventsourcing.cleanup --retain 5
# Deleted 150 old snapshots
```

#### Test Requirements
- [ ] Test snapshot save and load
- [ ] Test snapshot + events rehydration
- [ ] Test fallback without snapshot
- [ ] Test automatic snapshot creation
- [ ] Test snapshot cleanup
- [ ] Benchmark: load with 1M events + snapshot < 100ms
