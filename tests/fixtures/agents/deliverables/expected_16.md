---
id: "test-016"
title: "Message Queue Library"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a message queue library with support for multiple backends (Redis, RabbitMQ, SQS). Provides unified API, dead letter queues, and observability.

## Constraints

- Throughput: 50,000 messages/second
- At-least-once delivery guarantee

## Implementation Notes

- TypeScript library
- Backend adapter pattern
- Prometheus metrics

## Review Notes

(none yet)

## Tickets

### Ticket 1: Core Queue Interface

**Summary:** Define unified queue API and message handling.

**Definition of Done:** Interface works with mock backend.

#### Acceptance Criteria

1. **Queue Interface**
   - [ ] `publish(queue, message, options)` sends message
   - [ ] `subscribe(queue, handler, options)` consumes messages
   - [ ] `ack(message)` acknowledges processing
   - [ ] `nack(message, requeue)` negative acknowledgment

2. **Message Structure**
   - [ ] id: unique message identifier
   - [ ] body: serialized payload (JSON or Buffer)
   - [ ] headers: key-value metadata
   - [ ] timestamp: when message was published
   - [ ] attempts: delivery attempt count

3. **Publishing Options**
   - [ ] delay: schedule message for future delivery
   - [ ] ttl: message expiration time
   - [ ] priority: message priority (0-9)
   - [ ] deduplicationId: prevent duplicate messages

4. **Subscription Options**
   - [ ] concurrency: parallel message handlers
   - [ ] prefetch: messages to fetch at once
   - [ ] visibilityTimeout: processing time limit
   - [ ] maxRetries: retry limit before dead letter

5. **Connection Management**
   - [ ] `connect()` establishes connection
   - [ ] `disconnect()` gracefully closes
   - [ ] Automatic reconnection with backoff
   - [ ] Connection pooling

#### Demo Script
```typescript
import { Queue, Message } from '@company/queue';

// Create queue instance
const queue = new Queue({
  backend: 'redis',
  redis: { url: 'redis://localhost:6379' }
});

await queue.connect();

// Publish message
await queue.publish('orders', {
  orderId: '12345',
  items: [{ sku: 'ABC', qty: 2 }]
}, {
  delay: 5000,  // Deliver in 5 seconds
  headers: { 'x-priority': 'high' }
});

// Subscribe to messages
queue.subscribe('orders', async (message: Message) => {
  console.log('Processing order:', message.body.orderId);

  try {
    await processOrder(message.body);
    await queue.ack(message);
  } catch (error) {
    await queue.nack(message, true);  // Requeue
  }
}, {
  concurrency: 10,
  visibilityTimeout: 30000
});

// Graceful shutdown
process.on('SIGTERM', async () => {
  await queue.disconnect();
});
```

#### Test Requirements
- [ ] Unit tests with mock backend
- [ ] Test publish and subscribe flow
- [ ] Test acknowledgment handling
- [ ] Test retry logic
- [ ] Test connection management
- [ ] Test graceful shutdown

### Ticket 2: Backend Adapters

**Summary:** Implement Redis, RabbitMQ, and SQS adapters.

**Definition of Done:** Each backend passes integration tests.

#### Acceptance Criteria

1. **Redis Adapter**
   - [ ] Use Redis Streams for queue
   - [ ] Consumer groups for competing consumers
   - [ ] Pending messages tracking
   - [ ] XCLAIM for message recovery
   - [ ] Lua scripts for atomic operations

2. **RabbitMQ Adapter**
   - [ ] AMQP 0.9.1 protocol
   - [ ] Durable queues and messages
   - [ ] Publisher confirms
   - [ ] Consumer prefetch
   - [ ] Exchange bindings for routing

3. **SQS Adapter**
   - [ ] Standard and FIFO queue support
   - [ ] Long polling for efficiency
   - [ ] Batch send and receive
   - [ ] Message deduplication (FIFO)
   - [ ] Visibility timeout management

4. **Dead Letter Queue**
   - [ ] Automatic DLQ creation
   - [ ] Move messages after max retries
   - [ ] DLQ inspection API
   - [ ] Requeue from DLQ

5. **Feature Parity**
   - [ ] All backends support core interface
   - [ ] Graceful degradation for unsupported features
   - [ ] Feature detection API

#### Demo Script
```typescript
// Redis backend
const redisQueue = new Queue({
  backend: 'redis',
  redis: {
    url: 'redis://localhost:6379',
    streamMaxLen: 100000
  }
});

// RabbitMQ backend
const rabbitQueue = new Queue({
  backend: 'rabbitmq',
  rabbitmq: {
    url: 'amqp://localhost:5672',
    exchange: 'app-events',
    exchangeType: 'topic'
  }
});

// SQS backend
const sqsQueue = new Queue({
  backend: 'sqs',
  sqs: {
    region: 'us-east-1',
    queuePrefix: 'prod-',
    fifo: true
  }
});

// Same code works with any backend
async function sendOrder(queue: Queue, order: Order) {
  await queue.publish('orders', order);
}

// Check dead letter queue
const dlqMessages = await queue.deadLetterQueue('orders').peek(10);
for (const msg of dlqMessages) {
  console.log('Failed message:', msg.id, msg.body);
  await queue.deadLetterQueue('orders').requeue(msg);
}
```

#### Test Requirements
- [ ] Integration tests for each backend
- [ ] Test competing consumers
- [ ] Test message recovery after crash
- [ ] Test dead letter queue flow
- [ ] Load test: 50,000 msg/s throughput
- [ ] Test backend failover

### Ticket 3: Observability

**Summary:** Add metrics, logging, and tracing.

**Definition of Done:** Full visibility into queue operations.

#### Acceptance Criteria

1. **Prometheus Metrics**
   - [ ] messages_published_total{queue, status}
   - [ ] messages_consumed_total{queue, status}
   - [ ] message_processing_duration_seconds{queue}
   - [ ] queue_depth{queue} (gauge)
   - [ ] dead_letter_queue_depth{queue}

2. **Logging**
   - [ ] Structured JSON logs
   - [ ] Log levels: debug, info, warn, error
   - [ ] Include message ID, queue, operation
   - [ ] Configurable logger (winston, pino, etc.)

3. **Distributed Tracing**
   - [ ] OpenTelemetry integration
   - [ ] Trace context propagated in message headers
   - [ ] Span for publish operation
   - [ ] Span for consume operation
   - [ ] Link publish and consume spans

4. **Health Checks**
   - [ ] `healthCheck()` returns backend connectivity
   - [ ] Include queue statistics
   - [ ] Kubernetes liveness/readiness probes

5. **Debugging Tools**
   - [ ] Message inspection (peek without consume)
   - [ ] Queue statistics (depth, consumer count)
   - [ ] Slow consumer detection
   - [ ] Message replay for debugging

#### Demo Script
```typescript
import { Queue, PrometheusExporter, OpenTelemetryPlugin } from '@company/queue';

// Configure with observability
const queue = new Queue({
  backend: 'redis',
  redis: { url: 'redis://localhost:6379' },
  plugins: [
    new PrometheusExporter({ port: 9090 }),
    new OpenTelemetryPlugin({ serviceName: 'order-processor' })
  ],
  logging: {
    level: 'info',
    logger: console  // Or custom logger
  }
});

// Metrics endpoint
// GET http://localhost:9090/metrics
// messages_published_total{queue="orders",status="success"} 1234
// message_processing_duration_seconds{queue="orders",quantile="0.99"} 0.045

// Health check
const health = await queue.healthCheck();
// {
//   status: 'healthy',
//   backend: 'redis',
//   connected: true,
//   queues: {
//     orders: { depth: 150, consumers: 4 }
//   }
// }

// Debug: peek at messages
const pending = await queue.debug('orders').peek(5);
console.log('Pending messages:', pending);

// Debug: queue stats
const stats = await queue.debug('orders').stats();
// { depth: 150, consumers: 4, processingRate: 250, avgLatency: 45 }
```

#### Test Requirements
- [ ] Test Prometheus metrics accuracy
- [ ] Test structured logging output
- [ ] Test trace context propagation
- [ ] Test health check endpoint
- [ ] Test debug tools
- [ ] Verify trace spans in Jaeger/Zipkin
