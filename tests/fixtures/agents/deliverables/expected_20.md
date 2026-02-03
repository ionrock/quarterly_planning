---
id: "test-020"
title: "Search Indexing Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a search indexing service for full-text search across documents. Supports near-real-time indexing, faceted search, and typo tolerance.

## Constraints

- Index updates visible within 1 second
- Query latency under 100ms

## Implementation Notes

- Elasticsearch 8.x
- Debezium for CDC
- Kafka for indexing pipeline

## Review Notes

(none yet)

## Tickets

### Ticket 1: Index Management

**Summary:** Create and configure search indices.

**Definition of Done:** Indices created with proper mappings.

#### Acceptance Criteria

1. **Index Configuration**
   - [ ] Define index via JSON configuration file
   - [ ] Specify: name, settings, mappings
   - [ ] Support for aliases
   - [ ] Configurable shards and replicas

2. **Field Mappings**
   - [ ] text: full-text searchable
   - [ ] keyword: exact match, aggregations
   - [ ] date: date/time fields
   - [ ] numeric: integer, float for range queries
   - [ ] nested: for complex objects

3. **Analyzers**
   - [ ] Standard analyzer for general text
   - [ ] Language-specific analyzers
   - [ ] Custom analyzer configuration
   - [ ] Synonym support

4. **Index Lifecycle**
   - [ ] Create index with mappings
   - [ ] Update mappings (additive only)
   - [ ] Reindex for mapping changes
   - [ ] Delete index

5. **Index Aliasing**
   - [ ] Alias points to current index
   - [ ] Zero-downtime reindexing via alias swap
   - [ ] Read/write alias separation

#### Demo Script
```bash
# Create index configuration
cat > products_index.json << 'EOF'
{
  "settings": {
    "number_of_shards": 3,
    "number_of_replicas": 1,
    "analysis": {
      "analyzer": {
        "product_analyzer": {
          "type": "custom",
          "tokenizer": "standard",
          "filter": ["lowercase", "asciifolding", "product_synonyms"]
        }
      },
      "filter": {
        "product_synonyms": {
          "type": "synonym",
          "synonyms": ["laptop,notebook", "phone,mobile,cellphone"]
        }
      }
    }
  },
  "mappings": {
    "properties": {
      "name": {"type": "text", "analyzer": "product_analyzer"},
      "description": {"type": "text"},
      "price": {"type": "float"},
      "category": {"type": "keyword"},
      "tags": {"type": "keyword"},
      "created_at": {"type": "date"}
    }
  }
}
EOF

# Create index
curl -X POST http://localhost:8000/api/indices \
  -H "Content-Type: application/json" \
  -d @products_index.json
# Response: {"index": "products_v1", "alias": "products", "status": "created"}

# Update mappings (add field)
curl -X PUT http://localhost:8000/api/indices/products/mappings \
  -d '{"properties": {"brand": {"type": "keyword"}}}'

# List indices
curl http://localhost:8000/api/indices
# Response: [{"name": "products_v1", "alias": "products", "docs": 50000, "size": "120mb"}]
```

#### Test Requirements
- [ ] Test index creation with mappings
- [ ] Test analyzer configuration
- [ ] Test mapping updates
- [ ] Test alias management
- [ ] Test zero-downtime reindex
- [ ] Test invalid mapping rejection

### Ticket 2: Indexing Pipeline

**Summary:** Real-time document indexing from database.

**Definition of Done:** Changes indexed within 1 second.

#### Acceptance Criteria

1. **CDC Integration**
   - [ ] Debezium captures PostgreSQL changes
   - [ ] Captures: INSERT, UPDATE, DELETE
   - [ ] Handles schema changes gracefully
   - [ ] Offset tracking for resumption

2. **Kafka Pipeline**
   - [ ] Changes published to Kafka topic
   - [ ] Topic partitioned by document ID
   - [ ] Consumer group for processing
   - [ ] At-least-once delivery

3. **Document Transformation**
   - [ ] Map database columns to index fields
   - [ ] Transform nested relations
   - [ ] Compute derived fields
   - [ ] Handle NULL values

4. **Indexing**
   - [ ] Bulk indexing for efficiency
   - [ ] Flush batch every 100ms or 1000 docs
   - [ ] Handle indexing errors (retry, dead letter)
   - [ ] Track indexing lag

5. **Deletion Handling**
   - [ ] Soft delete: update document, mark deleted
   - [ ] Hard delete: remove from index
   - [ ] Cascading deletes (related documents)

#### Demo Script
```bash
# Start indexing pipeline
./indexer start --config pipeline.yaml

# Pipeline config
# pipeline.yaml:
# source:
#   type: debezium
#   postgres:
#     host: localhost
#     database: myapp
#     tables: [products, categories]
#
# transform:
#   products:
#     mappings:
#       name: product_name
#       price: unit_price
#     joins:
#       - table: categories
#         on: category_id
#         fields: [category_name]
#
# sink:
#   type: elasticsearch
#   index: products
#   bulk_size: 1000
#   flush_interval: 100ms

# Check pipeline status
curl http://localhost:8000/api/pipeline/status
# {
#   "status": "running",
#   "lag_ms": 150,
#   "docs_indexed": 1500000,
#   "errors_24h": 5,
#   "throughput": "5000 docs/s"
# }

# Manual reindex
curl -X POST http://localhost:8000/api/pipeline/reindex \
  -d '{"table": "products", "since": "2024-01-01"}'
# Response: {"job_id": "reindex_123", "estimated_docs": 50000}

# Insert row in database
psql -c "INSERT INTO products (name, price) VALUES ('Widget', 29.99)"

# Verify indexed (within 1 second)
curl "http://localhost:8000/api/search/products?q=Widget"
# Response includes the new product
```

#### Test Requirements
- [ ] Test CDC capture (INSERT, UPDATE, DELETE)
- [ ] Test document transformation
- [ ] Test bulk indexing
- [ ] Test error handling and retry
- [ ] Test indexing latency < 1 second
- [ ] Test full reindex
- [ ] Test pipeline restart/recovery

### Ticket 3: Search API

**Summary:** Query interface with filters and facets.

**Definition of Done:** Search returns relevant results quickly.

#### Acceptance Criteria

1. **Full-Text Search**
   - [ ] Multi-field search
   - [ ] Phrase matching
   - [ ] Fuzzy matching (typo tolerance)
   - [ ] Boosting by field importance

2. **Filtering**
   - [ ] Exact match: category = "Electronics"
   - [ ] Range: price >= 100 AND price <= 500
   - [ ] Terms: tags IN ["sale", "new"]
   - [ ] Boolean: AND, OR, NOT

3. **Faceted Search**
   - [ ] Count by category
   - [ ] Count by tag
   - [ ] Price ranges (histograms)
   - [ ] Facets filter each other

4. **Sorting**
   - [ ] Sort by relevance (default)
   - [ ] Sort by field: price, date
   - [ ] Multiple sort criteria
   - [ ] Sort direction: asc, desc

5. **Pagination**
   - [ ] Offset-based: from, size
   - [ ] Cursor-based for deep pagination
   - [ ] Total count included

6. **Highlighting**
   - [ ] Matched terms highlighted
   - [ ] Configurable highlight tags
   - [ ] Fragment size configurable

#### Demo Script
```bash
# Basic search
curl -X POST http://localhost:8000/api/search/products \
  -d '{
    "query": "wireless headphones",
    "size": 10
  }'
# Response:
# {
#   "hits": [
#     {"_id": "1", "name": "Premium Wireless Headphones", "price": 299.99, "score": 8.5},
#     ...
#   ],
#   "total": 150,
#   "took_ms": 25
# }

# Search with filters and facets
curl -X POST http://localhost:8000/api/search/products \
  -d '{
    "query": "headphones",
    "filters": {
      "category": "Electronics",
      "price": {"gte": 50, "lte": 200}
    },
    "facets": ["category", "brand", "price_range"],
    "sort": [{"price": "asc"}],
    "highlight": true
  }'
# Response:
# {
#   "hits": [...],
#   "facets": {
#     "category": [{"value": "Electronics", "count": 50}, ...],
#     "brand": [{"value": "Sony", "count": 20}, ...],
#     "price_range": [{"from": 50, "to": 100, "count": 30}, ...]
#   },
#   "total": 50,
#   "took_ms": 35
# }

# Fuzzy search (typo tolerance)
curl -X POST http://localhost:8000/api/search/products \
  -d '{"query": "headhpones", "fuzzy": true}'
# Still finds "headphones"

# Autocomplete
curl "http://localhost:8000/api/search/products/suggest?prefix=wire"
# Response: ["wireless headphones", "wireless earbuds", "wireless charger"]
```

#### Test Requirements
- [ ] Test full-text search relevance
- [ ] Test all filter types
- [ ] Test facet accuracy
- [ ] Test sorting
- [ ] Test pagination
- [ ] Test highlighting
- [ ] Test typo tolerance
- [ ] Benchmark: query latency < 100ms
