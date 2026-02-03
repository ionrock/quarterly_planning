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

#### Steps

1. **Create Python package**
   - Create eventsourcing/ directory
   - Create pyproject.toml
   - Verify: package installs

2. **Install dependencies**
   - Add asyncpg for PostgreSQL
   - Add msgpack for serialization
   - Verify: packages install

3. **Define Event base class**
   - Create eventsourcing/events.py
   - Include: event_id, event_type, timestamp, data, metadata
   - Verify: class works

4. **Create database schema**
   - Create events table: stream_id, version, event_id, event_type, data, metadata, timestamp
   - Add unique constraint on (stream_id, version)
   - Create streams table: id, version
   - Verify: schema valid

5. **Create migration script**
   - SQL to create tables
   - SQL to rollback
   - Verify: migration works

6. **Define EventStore interface**
   - Create eventsourcing/store.py
   - Define abstract append(), read(), read_all() methods
   - Verify: interface compiles

7. **Implement PostgreSQL event store**
   - Create eventsourcing/postgres.py
   - Accept asyncpg connection pool
   - Verify: store initializes

8. **Implement append() method**
   - Begin transaction
   - Check expected version matches
   - Insert events with incrementing version
   - Update stream version
   - Commit transaction
   - Verify: append works

9. **Implement optimistic concurrency**
   - Raise ConcurrencyError if version mismatch
   - Include expected vs actual in error
   - Verify: concurrent writes detected

10. **Implement read() method**
    - Query events by stream_id
    - Support from_version and to_version
    - Order by version
    - Verify: read works

11. **Implement read_all() method**
    - Query all events across streams
    - Support pagination with position
    - Verify: global read works

12. **Serialize events with msgpack**
    - Serialize data and metadata fields
    - Deserialize on read
    - Verify: serialization works

### Ticket 2: Aggregate Framework

**Summary:** Create base class for aggregates.

**Definition of Done:** Aggregates apply events and track changes.

#### Steps

1. **Define Aggregate base class**
   - Create eventsourcing/aggregate.py
   - Include: id, version, _pending_events list
   - Verify: class compiles

2. **Implement apply() abstract method**
   - Called for each event
   - Updates aggregate state
   - Verify: apply defined

3. **Implement _record() method**
   - Create and apply new event
   - Add to _pending_events
   - Verify: events recorded

4. **Implement clear_pending() method**
   - Return and clear pending events
   - Used after successful save
   - Verify: pending cleared

5. **Implement from_events() class method**
   - Create new aggregate instance
   - Apply each event in sequence
   - Update version counter
   - Verify: reconstruction works

6. **Create Repository class**
   - Create eventsourcing/repository.py
   - Accept event store and aggregate class
   - Verify: repository initializes

7. **Implement load() method**
   - Query events from store
   - Reconstruct aggregate
   - Verify: load works

8. **Implement save() method**
   - Get pending events
   - Append to store with expected version
   - Clear pending on success
   - Verify: save works

9. **Handle empty aggregate**
   - Return new instance if no events
   - Verify: new aggregates work

10. **Add typing support**
    - Add type hints throughout
    - Use Generic[T] for repository
    - Verify: mypy passes

11. **Write example aggregate**
    - Create BankAccount example
    - Implement deposit, withdraw, apply
    - Verify: example works

12. **Test event replay**
    - Create aggregate, make changes
    - Save and reload
    - Verify state matches
    - Verify: replay correct

### Ticket 3: Snapshot Support

**Summary:** Add snapshotting for performance.

**Definition of Done:** Aggregates load from snapshots.

#### Steps

1. **Define Snapshot class**
   - Include: aggregate_id, version, state (bytes), created_at
   - Verify: class compiles

2. **Create snapshots table**
   - Columns: aggregate_id, version, state, created_at
   - Index on aggregate_id
   - Verify: table created

3. **Define SnapshotStore interface**
   - Define save(), load() methods
   - Verify: interface compiles

4. **Implement PostgreSQL snapshot store**
   - Create eventsourcing/snapshot_postgres.py
   - Implement save and load
   - Verify: snapshot store works

5. **Serialize aggregate state**
   - Use pickle or custom serialization
   - Handle non-picklable attributes
   - Verify: serialization works

6. **Modify repository for snapshots**
   - Accept optional snapshot_store
   - Configure snapshot_frequency (e.g., every 100 events)
   - Verify: config works

7. **Load from snapshot on repository.load()**
   - Try to load latest snapshot first
   - Apply only events after snapshot version
   - Verify: snapshot loading works

8. **Save snapshot periodically**
   - After save(), check if version % frequency == 0
   - Create and save snapshot
   - Verify: snapshots created

9. **Handle missing snapshot gracefully**
   - Fall back to full replay if no snapshot
   - Verify: missing snapshot handled

10. **Add snapshot cleanup**
    - Keep only N most recent snapshots
    - Delete old snapshots
    - Verify: cleanup works

11. **Benchmark performance**
    - Create aggregate with 10,000 events
    - Measure load time with and without snapshot
    - Verify: significant speedup

12. **Document snapshot usage**
    - Document when to use snapshots
    - Document configuration
    - Verify: docs complete

13. **Test snapshot with large aggregates**
    - Create aggregate with 1 million events
    - Verify snapshot enables loading
    - Verify: large aggregates work
