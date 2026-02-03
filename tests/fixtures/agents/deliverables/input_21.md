---
id: "test-021"
title: "Email Sending Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build an email sending service with templating, queue management, and delivery tracking. Supports transactional and bulk email with high deliverability.

## Constraints

- Send 100,000 emails per hour
- 95%+ inbox placement rate

## Implementation Notes

- Node.js with Nodemailer
- Multiple ESP support (SES, SendGrid)
- PostgreSQL for tracking

## Review Notes

(none yet)

## Tickets

### Ticket 1: Email Templates

**Summary:** HTML/text email templates with variables.

**Definition of Done:** Templates render correctly in all clients.

### Ticket 2: Sending Engine

**Summary:** Queue and send emails via ESP.

**Definition of Done:** Emails sent reliably with failover.

### Ticket 3: Delivery Tracking

**Summary:** Track opens, clicks, and bounces.

**Definition of Done:** Delivery events recorded accurately.
