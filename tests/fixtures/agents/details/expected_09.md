---
id: "test-009"
title: "Notification Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a notification service that sends emails, SMS, and push notifications. Other services publish events, and this service routes them to appropriate channels based on user preferences. Built with Go.

## Constraints

- Deliver notifications within 5 seconds
- Support at least 10,000 notifications per minute

## Implementation Notes

### Technology Stack
- **Language:** Go 1.21+
- **Message Queue:** RabbitMQ with amqp091-go client
- **Database:** PostgreSQL with pgx driver
- **Providers:** SendGrid API v3, Twilio REST API, Firebase Admin SDK

### Event Message Schema
```go
type NotificationEvent struct {
    ID          string                 `json:"id"`
    UserID      string                 `json:"user_id"`
    Type        string                 `json:"type"`  // e.g., "order_confirmed", "password_reset"
    Priority    Priority               `json:"priority"` // low, normal, high, critical
    Data        map[string]interface{} `json:"data"`
    CreatedAt   time.Time              `json:"created_at"`
    IdempotencyKey string              `json:"idempotency_key"`
}

type Priority string
const (
    PriorityLow      Priority = "low"
    PriorityNormal   Priority = "normal"
    PriorityHigh     Priority = "high"
    PriorityCritical Priority = "critical"
)
```

### User Preferences Schema
```sql
CREATE TABLE notification_preferences (
    user_id UUID PRIMARY KEY,
    email_enabled BOOLEAN NOT NULL DEFAULT true,
    sms_enabled BOOLEAN NOT NULL DEFAULT false,
    push_enabled BOOLEAN NOT NULL DEFAULT true,
    quiet_hours_start TIME,
    quiet_hours_end TIME,
    timezone VARCHAR(50) DEFAULT 'UTC',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE notification_subscriptions (
    user_id UUID NOT NULL,
    notification_type VARCHAR(100) NOT NULL,
    channel VARCHAR(20) NOT NULL,  -- 'email', 'sms', 'push'
    enabled BOOLEAN NOT NULL DEFAULT true,
    PRIMARY KEY (user_id, notification_type, channel)
);
```

### Provider Interface
```go
type NotificationProvider interface {
    Send(ctx context.Context, notification Notification) error
    Channel() Channel
}

type Channel string
const (
    ChannelEmail Channel = "email"
    ChannelSMS   Channel = "sms"
    ChannelPush  Channel = "push"
)

type Notification struct {
    To       string
    Subject  string
    Body     string
    BodyHTML string
    Data     map[string]string
}
```

### RabbitMQ Consumer
```go
func (s *Service) StartConsumer(ctx context.Context) error {
    msgs, err := s.channel.Consume(
        "notifications",  // queue
        "",              // consumer tag
        false,           // auto-ack (manual for reliability)
        false,           // exclusive
        false,           // no-local
        false,           // no-wait
        nil,             // args
    )
    if err != nil {
        return fmt.Errorf("consume: %w", err)
    }

    // Process with worker pool
    for i := 0; i < s.workerCount; i++ {
        go s.worker(ctx, msgs)
    }

    <-ctx.Done()
    return nil
}

func (s *Service) worker(ctx context.Context, msgs <-chan amqp.Delivery) {
    for msg := range msgs {
        var event NotificationEvent
        if err := json.Unmarshal(msg.Body, &event); err != nil {
            log.Error().Err(err).Msg("unmarshal event")
            msg.Nack(false, false)  // Don't requeue malformed messages
            continue
        }

        if err := s.processEvent(ctx, event); err != nil {
            log.Error().Err(err).Str("event_id", event.ID).Msg("process event")
            msg.Nack(false, true)  // Requeue for retry
            continue
        }

        msg.Ack(false)
    }
}
```

### Routing Logic
```go
func (s *Service) processEvent(ctx context.Context, event NotificationEvent) error {
    // Check idempotency
    if s.isDuplicate(ctx, event.IdempotencyKey) {
        return nil
    }

    // Get user preferences
    prefs, err := s.prefRepo.GetByUserID(ctx, event.UserID)
    if err != nil {
        return fmt.Errorf("get preferences: %w", err)
    }

    // Get enabled channels for this notification type
    channels := s.getEnabledChannels(prefs, event.Type)

    // Render templates
    notification := s.templateEngine.Render(event.Type, event.Data)

    // Send to each channel
    var errs []error
    for _, channel := range channels {
        provider := s.providers[channel]
        if err := provider.Send(ctx, notification); err != nil {
            errs = append(errs, fmt.Errorf("%s: %w", channel, err))
        }
    }

    // Record delivery
    s.recordDelivery(ctx, event, channels, errs)

    if len(errs) > 0 {
        return fmt.Errorf("delivery errors: %v", errs)
    }
    return nil
}
```

### Retry Configuration
```go
type RetryConfig struct {
    MaxAttempts     int           // 3
    InitialBackoff  time.Duration // 1 minute
    MaxBackoff      time.Duration // 1 hour
    BackoffMultiplier float64     // 2.0
}

// Dead letter queue for failed messages after max retries
// DLQ: notifications.dlq
```

### Provider Implementations (abbreviated)
```go
// SendGrid
func (p *SendGridProvider) Send(ctx context.Context, n Notification) error {
    message := mail.NewSingleEmail(
        mail.NewEmail(p.fromName, p.fromEmail),
        n.Subject,
        mail.NewEmail("", n.To),
        n.Body,
        n.BodyHTML,
    )
    _, err := p.client.Send(message)
    return err
}

// Twilio
func (p *TwilioProvider) Send(ctx context.Context, n Notification) error {
    params := &openapi.CreateMessageParams{}
    params.SetTo(n.To)
    params.SetFrom(p.fromNumber)
    params.SetBody(n.Body)
    _, err := p.client.Api.CreateMessage(params)
    return err
}
```

### Metrics
- `notifications_sent_total{channel,type,status}` - Counter
- `notification_latency_seconds{channel}` - Histogram
- `notification_queue_depth` - Gauge

## Review Notes

(none yet)

## Tickets

### Ticket 1: Event Consumer

**Summary:** Consume notification events from RabbitMQ.

**Definition of Done:** Events are consumed and parsed correctly.

### Ticket 2: Provider Integrations

**Summary:** Integrate with SendGrid, Twilio, and Firebase.

**Definition of Done:** Notifications are sent through each provider.

### Ticket 3: Preference Management

**Summary:** Store and query user notification preferences.

**Definition of Done:** Notifications respect user preferences.
