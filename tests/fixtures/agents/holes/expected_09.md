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

- Use message queue (RabbitMQ) for event ingestion
- Integrate with SendGrid for email, Twilio for SMS, Firebase for push
- Store user notification preferences in PostgreSQL
- Implement retry logic for failed deliveries

## Review Notes

### Identified Weaknesses

1. **No deduplication**: What prevents the same notification from being sent multiple times?

2. **Template management missing**: Where are notification templates stored and managed?

3. **No opt-out/unsubscribe handling**: Legal compliance (CAN-SPAM, GDPR) requires easy opt-out.

4. **Missing delivery tracking**: How do we know if notifications were actually delivered?

5. **No priority/urgency levels**: All notifications treated equally; critical alerts could be delayed.

### Edge Cases

- What if a user has no valid channels (unverified email, no phone)?
- How are bounced emails or invalid phone numbers handled?
- What about timezone-aware delivery (don't SMS at 3am)?
- Rate limiting per user (prevent notification spam)?
- What if a user changes preferences while notifications are in flight?
- How are localization/translations handled?

### Assumptions to Validate

- Are SendGrid/Twilio/Firebase accounts already set up?
- What's the notification event schema from publishing services?
- Is there a default preference for new users?
- Do we need delivery receipts/read receipts?
- What's the retention policy for notification history?

### Potential Failures

- RabbitMQ unavailability (messages lost or stuck)
- Provider API rate limits exceeded
- Database connection failures blocking preference lookup
- Invalid/expired push notification tokens
- Provider outages (need failover providers?)
- Message queue backlog during traffic spikes

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
