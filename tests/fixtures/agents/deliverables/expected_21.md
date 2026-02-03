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

#### Acceptance Criteria

1. **Template Storage**
   - [ ] Templates stored with name and version
   - [ ] Both HTML and plain text versions
   - [ ] Template metadata: subject, from, reply-to
   - [ ] Version history maintained

2. **Variable Substitution**
   - [ ] {{variable}} syntax for simple values
   - [ ] Nested access: {{user.name}}
   - [ ] Loops: {{#each items}}
   - [ ] Conditionals: {{#if condition}}

3. **Template Design**
   - [ ] Responsive design (mobile-friendly)
   - [ ] Inline CSS (for email client compatibility)
   - [ ] Image hosting with CDN URLs
   - [ ] Dark mode support (prefers-color-scheme)

4. **Preview and Testing**
   - [ ] Preview template with sample data
   - [ ] Send test email to address
   - [ ] Render preview for different clients
   - [ ] Spam score check

5. **Template Validation**
   - [ ] Required variables documented
   - [ ] Validation against sample data
   - [ ] HTML syntax validation
   - [ ] Link validation

#### Demo Script
```bash
# Create template
curl -X POST http://localhost:8000/api/templates \
  -H "Content-Type: application/json" \
  -d '{
    "name": "welcome",
    "subject": "Welcome to {{company_name}}, {{user.first_name}}!",
    "html": "<html>...<h1>Welcome, {{user.first_name}}!</h1>...</html>",
    "text": "Welcome, {{user.first_name}}!\n\nThank you for joining...",
    "variables": {
      "user": {"first_name": "string", "email": "string"},
      "company_name": "string"
    }
  }'
# Response: {"id": "tpl_abc123", "version": 1}

# Preview template
curl -X POST http://localhost:8000/api/templates/tpl_abc123/preview \
  -d '{
    "data": {"user": {"first_name": "Alice"}, "company_name": "Acme Inc"}
  }'
# Response: {"html": "<html>...<h1>Welcome, Alice!</h1>...</html>", "text": "..."}

# Send test email
curl -X POST http://localhost:8000/api/templates/tpl_abc123/test \
  -d '{
    "to": "test@example.com",
    "data": {"user": {"first_name": "Alice"}, "company_name": "Acme Inc"}
  }'
# Response: {"message_id": "msg_123", "status": "sent"}

# Check spam score
curl http://localhost:8000/api/templates/tpl_abc123/spam-check
# Response: {"score": 0.2, "issues": [], "verdict": "likely_inbox"}
```

#### Test Requirements
- [ ] Test variable substitution
- [ ] Test loops and conditionals
- [ ] Test HTML/text generation
- [ ] Test responsive design (mobile viewport)
- [ ] Test rendering in email clients (Litmus/Email on Acid)
- [ ] Test spam score calculation

### Ticket 2: Sending Engine

**Summary:** Queue and send emails via ESP.

**Definition of Done:** Emails sent reliably with failover.

#### Acceptance Criteria

1. **Email Submission**
   - [ ] POST /api/send accepts email request
   - [ ] Required: to, template, data
   - [ ] Optional: from, cc, bcc, attachments, schedule
   - [ ] Returns message ID immediately

2. **Queue Management**
   - [ ] Emails queued for async sending
   - [ ] Priority queues (transactional, marketing)
   - [ ] Rate limiting per recipient domain
   - [ ] Deduplication by idempotency key

3. **ESP Integration**
   - [ ] Primary ESP: Amazon SES
   - [ ] Fallback ESP: SendGrid
   - [ ] Automatic failover on errors
   - [ ] Per-domain ESP routing (optional)

4. **Sending Features**
   - [ ] Attachments (up to 10MB)
   - [ ] Inline images (cid: references)
   - [ ] Custom headers
   - [ ] Scheduled sending

5. **Error Handling**
   - [ ] Retry on temporary failures
   - [ ] Mark permanent failures (invalid email)
   - [ ] Bounce handling (update recipient status)
   - [ ] Complaint handling (unsubscribe)

#### Demo Script
```bash
# Send single email
curl -X POST http://localhost:8000/api/send \
  -H "Content-Type: application/json" \
  -d '{
    "to": "user@example.com",
    "template": "welcome",
    "data": {"user": {"first_name": "Alice"}, "company_name": "Acme Inc"},
    "priority": "high"
  }'
# Response: {"message_id": "msg_abc123", "status": "queued"}

# Send with attachment
curl -X POST http://localhost:8000/api/send \
  -F 'to=user@example.com' \
  -F 'template=invoice' \
  -F 'data={"invoice_number": "INV-001"}' \
  -F 'attachment=@invoice.pdf'

# Schedule email
curl -X POST http://localhost:8000/api/send \
  -d '{
    "to": "user@example.com",
    "template": "reminder",
    "data": {...},
    "scheduled_at": "2024-01-20T09:00:00Z"
  }'

# Send bulk emails
curl -X POST http://localhost:8000/api/send/bulk \
  -d '{
    "template": "newsletter",
    "recipients": [
      {"email": "a@example.com", "data": {"name": "Alice"}},
      {"email": "b@example.com", "data": {"name": "Bob"}}
    ]
  }'
# Response: {"batch_id": "batch_123", "count": 2}

# Check message status
curl http://localhost:8000/api/messages/msg_abc123
# Response: {"status": "delivered", "sent_at": "...", "esp": "ses"}
```

#### Test Requirements
- [ ] Test email submission and queuing
- [ ] Test ESP integration (SES, SendGrid)
- [ ] Test failover between ESPs
- [ ] Test rate limiting
- [ ] Test scheduled sending
- [ ] Test attachment handling
- [ ] Load test: 100,000 emails/hour

### Ticket 3: Delivery Tracking

**Summary:** Track opens, clicks, and bounces.

**Definition of Done:** Delivery events recorded accurately.

#### Acceptance Criteria

1. **Event Tracking**
   - [ ] Sent: email handed off to ESP
   - [ ] Delivered: ESP confirms delivery
   - [ ] Opened: tracking pixel loaded
   - [ ] Clicked: link clicked
   - [ ] Bounced: delivery failed
   - [ ] Complained: marked as spam

2. **Open Tracking**
   - [ ] 1x1 transparent tracking pixel
   - [ ] Unique URL per recipient
   - [ ] First open timestamp recorded
   - [ ] Open count tracked

3. **Click Tracking**
   - [ ] Links rewritten through tracking domain
   - [ ] Original URL preserved
   - [ ] Click timestamp and link recorded
   - [ ] Unique clicks vs total clicks

4. **Webhook Processing**
   - [ ] SES notification processing (SNS)
   - [ ] SendGrid event webhook
   - [ ] Idempotent event processing
   - [ ] Webhook signature verification

5. **Analytics API**
   - [ ] Message-level events
   - [ ] Aggregate stats per template
   - [ ] Time-series data
   - [ ] Export to data warehouse

#### Demo Script
```bash
# Get message events
curl http://localhost:8000/api/messages/msg_abc123/events
# Response:
# {
#   "events": [
#     {"type": "queued", "timestamp": "2024-01-15T10:00:00Z"},
#     {"type": "sent", "timestamp": "2024-01-15T10:00:01Z", "esp": "ses"},
#     {"type": "delivered", "timestamp": "2024-01-15T10:00:02Z"},
#     {"type": "opened", "timestamp": "2024-01-15T10:05:00Z", "ip": "..."},
#     {"type": "clicked", "timestamp": "2024-01-15T10:06:00Z", "url": "..."}
#   ]
# }

# Get template analytics
curl "http://localhost:8000/api/templates/welcome/analytics?period=7d"
# Response:
# {
#   "template": "welcome",
#   "period": "7d",
#   "stats": {
#     "sent": 5000,
#     "delivered": 4950,
#     "bounced": 50,
#     "opened": 2500,
#     "clicked": 1000,
#     "complained": 5
#   },
#   "rates": {
#     "delivery_rate": 0.99,
#     "open_rate": 0.505,
#     "click_rate": 0.202
#   }
# }

# Webhook endpoint (for ESP)
# POST http://localhost:8000/webhooks/ses
# POST http://localhost:8000/webhooks/sendgrid
```

#### Test Requirements
- [ ] Test tracking pixel generation
- [ ] Test link rewriting
- [ ] Test webhook processing (SES, SendGrid)
- [ ] Test event deduplication
- [ ] Test analytics accuracy
- [ ] Test privacy compliance (unsubscribe tracking)
