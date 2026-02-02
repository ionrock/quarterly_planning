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
