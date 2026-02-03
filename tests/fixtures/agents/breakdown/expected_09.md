---
id: "test-009"
title: "Notification Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a notification service that sends alerts via email, SMS, and push notifications. Supports templating, scheduling, and delivery tracking.

## Constraints

- Delivery within 30 seconds
- 99.9% delivery rate

## Implementation Notes

- Use SendGrid for email
- Twilio for SMS
- Firebase for push
- PostgreSQL for tracking

## Review Notes

(none yet)

## Tickets

### Ticket 1: Provider Integration

**Summary:** Connect to notification providers.

**Definition of Done:** Can send via all three channels.

#### Steps

1. **Create provider interface**
   - Create src/providers/base.ts
   - Define NotificationProvider interface with send() method
   - Define NotificationResult type
   - Verify: interface compiles

2. **Install SendGrid SDK**
   - Run `npm install @sendgrid/mail`
   - Verify: package installed

3. **Implement SendGrid provider**
   - Create src/providers/sendgrid.ts
   - Configure API key from SENDGRID_API_KEY env
   - Implement send() for email notifications
   - Verify: test email sends successfully

4. **Install Twilio SDK**
   - Run `npm install twilio`
   - Verify: package installed

5. **Implement Twilio provider**
   - Create src/providers/twilio.ts
   - Configure account SID and auth token from env
   - Implement send() for SMS notifications
   - Verify: test SMS sends successfully

6. **Install Firebase Admin SDK**
   - Run `npm install firebase-admin`
   - Verify: package installed

7. **Implement Firebase provider**
   - Create src/providers/firebase.ts
   - Initialize with service account from env
   - Implement send() for push notifications
   - Verify: test push sends successfully

8. **Create provider registry**
   - Create src/providers/index.ts
   - Map channel names to provider instances
   - Verify: getProvider('email') returns SendGrid

9. **Add provider health checks**
   - Implement healthCheck() on each provider
   - Verify: health check returns provider status

### Ticket 2: Template System

**Summary:** Create and render notification templates.

**Definition of Done:** Templates support variables and localization.

#### Steps

1. **Create templates database table**
   - Columns: id, name, channel, subject, body, locale, created_at
   - Unique constraint on (name, channel, locale)
   - Verify: migration runs

2. **Install templating library**
   - Run `npm install handlebars`
   - Verify: package installed

3. **Create template model**
   - Create src/models/template.ts
   - Define Template interface
   - Verify: interface compiles

4. **Create template repository**
   - Create src/repositories/templateRepository.ts
   - Implement findByName(name, channel, locale)
   - Implement create(), update(), delete()
   - Verify: CRUD operations work

5. **Implement template rendering**
   - Create src/templates/render.ts
   - Compile template with Handlebars
   - Replace variables with context data
   - Verify: variables replaced correctly

6. **Add template validation**
   - Validate required variables present in template
   - Validate Handlebars syntax
   - Verify: invalid templates rejected

7. **Create locale fallback chain**
   - Try exact locale (e.g., en-US)
   - Fall back to language (e.g., en)
   - Fall back to default locale
   - Verify: fallback chain works

8. **Create CRUD API for templates**
   - POST /templates - create template
   - GET /templates/:name - get template
   - PUT /templates/:id - update template
   - DELETE /templates/:id - delete template
   - Verify: API endpoints work

9. **Add template preview endpoint**
   - POST /templates/:name/preview with context
   - Return rendered template without sending
   - Verify: preview shows rendered content

### Ticket 3: Delivery Tracking

**Summary:** Track delivery status and retries.

**Definition of Done:** Delivery status queryable, failures retried.

#### Steps

1. **Create notifications table**
   - Columns: id, channel, recipient, template_name, context, status, attempts, last_attempt_at, delivered_at, created_at
   - Index on (status, created_at)
   - Verify: migration runs

2. **Create notification model**
   - Create src/models/notification.ts
   - Define Notification interface with status enum
   - Status: pending, sending, delivered, failed
   - Verify: interface compiles

3. **Create notification repository**
   - Create src/repositories/notificationRepository.ts
   - Implement create(), updateStatus(), findPending()
   - Verify: operations work

4. **Create notification queue**
   - Install and configure BullMQ
   - Create 'notifications' queue
   - Verify: queue created

5. **Create notification worker**
   - Create src/workers/notificationWorker.ts
   - Process jobs from queue
   - Render template and send via provider
   - Verify: worker processes jobs

6. **Update status on send**
   - Set status to 'sending' when processing
   - Set status to 'delivered' on success
   - Set status to 'failed' on error
   - Verify: status updates correctly

7. **Implement retry logic**
   - Configure exponential backoff: 1m, 5m, 15m, 1h
   - Maximum 4 retry attempts
   - Verify: failed notifications retried

8. **Record delivery attempts**
   - Increment attempts count on each try
   - Update last_attempt_at timestamp
   - Store error message on failure
   - Verify: attempt tracking works

9. **Create delivery status endpoint**
   - GET /notifications/:id returns status
   - Include attempt history
   - Verify: status queryable

10. **Handle provider webhooks**
    - POST /webhooks/sendgrid for email events
    - POST /webhooks/twilio for SMS events
    - Update delivery status from webhook
    - Verify: webhooks update status

11. **Add delivery metrics**
    - Track delivery rate by channel
    - Track average delivery time
    - Export to monitoring system
    - Verify: metrics visible

12. **Create failed notification alert**
    - Alert when delivery rate drops below 99%
    - Include affected notification IDs
    - Verify: alerts triggered
