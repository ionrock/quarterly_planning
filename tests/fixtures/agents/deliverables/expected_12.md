---
id: "test-012"
title: "Distributed Task Scheduler"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a distributed task scheduler for running jobs across multiple workers. Supports cron schedules, dependencies, and retries. Provides UI for monitoring.

## Constraints

- Handle 10,000 concurrent tasks
- Task execution exactly-once guarantee

## Implementation Notes

- Python backend with FastAPI
- PostgreSQL for job state
- Redis for distributed locking

## Review Notes

(none yet)

## Tickets

### Ticket 1: Job Definition

**Summary:** Define jobs with schedules and parameters.

**Definition of Done:** Jobs can be created and scheduled.

#### Acceptance Criteria

1. **Job Schema**
   - [ ] Job has: id, name, type, schedule, parameters, created_at
   - [ ] Type: python function, shell command, HTTP request
   - [ ] Parameters: JSON object passed to job
   - [ ] Tags for grouping and filtering

2. **Scheduling**
   - [ ] Cron expression: "0 9 * * *" (9 AM daily)
   - [ ] Interval: every 5m, 1h, 1d
   - [ ] One-time: specific datetime
   - [ ] Cron validated on job creation

3. **Job Dependencies**
   - [ ] Job can depend on other jobs
   - [ ] Dependent job runs after dependency succeeds
   - [ ] Circular dependencies detected and rejected
   - [ ] DAG visualization available

4. **Job Execution Settings**
   - [ ] Timeout: max execution time (default 1 hour)
   - [ ] Retries: max retry count (default 3)
   - [ ] Retry delay: fixed or exponential backoff
   - [ ] Concurrency limit: max parallel executions

5. **CRUD API**
   - [ ] POST /api/jobs creates job
   - [ ] GET /api/jobs lists jobs with filtering
   - [ ] PUT /api/jobs/{id} updates job
   - [ ] DELETE /api/jobs/{id} removes job

#### Demo Script
```bash
# Create a cron job
curl -X POST http://localhost:8000/api/jobs \
  -H "Content-Type: application/json" \
  -d '{
    "name": "daily-report",
    "type": "python",
    "function": "reports.generate_daily",
    "schedule": "0 9 * * *",
    "parameters": {"recipients": ["team@example.com"]},
    "timeout": 3600,
    "retries": 2
  }'
# Response: {"id": "job-123", "name": "daily-report", "next_run": "2024-01-16T09:00:00Z"}

# Create job with dependency
curl -X POST http://localhost:8000/api/jobs \
  -d '{
    "name": "send-report",
    "type": "python",
    "function": "email.send_report",
    "depends_on": ["daily-report"]
  }'

# List jobs
curl http://localhost:8000/api/jobs?tag=reports
# Response: {"jobs": [...], "total": 5}

# Trigger job manually
curl -X POST http://localhost:8000/api/jobs/job-123/trigger
# Response: {"run_id": "run-456", "status": "queued"}
```

#### Test Requirements
- [ ] Test job CRUD operations
- [ ] Test cron schedule parsing
- [ ] Test dependency validation
- [ ] Test circular dependency detection
- [ ] Test manual job trigger

### Ticket 2: Worker Pool

**Summary:** Distributed workers that execute jobs.

**Definition of Done:** Workers pick up and complete jobs.

#### Acceptance Criteria

1. **Worker Registration**
   - [ ] Worker registers with coordinator on startup
   - [ ] Heartbeat every 30 seconds
   - [ ] Worker capabilities: tags, concurrency limit
   - [ ] Automatic deregistration on shutdown

2. **Job Assignment**
   - [ ] Coordinator assigns jobs to available workers
   - [ ] Job locked with Redis to prevent double-execution
   - [ ] Lock expires after timeout (prevents stuck jobs)
   - [ ] Worker confirms job receipt

3. **Execution**
   - [ ] Worker executes job in subprocess
   - [ ] stdout/stderr captured and stored
   - [ ] Exit code determines success/failure
   - [ ] Timeout enforcement via SIGTERM then SIGKILL

4. **Exactly-Once Guarantee**
   - [ ] Job marked in-progress before execution
   - [ ] Distributed lock prevents duplicate execution
   - [ ] Idempotency key for HTTP jobs
   - [ ] Failed lock acquisition skips job

5. **Retry Logic**
   - [ ] Failed jobs re-queued up to retry limit
   - [ ] Exponential backoff: 1s, 2s, 4s, 8s...
   - [ ] Retries use same idempotency key
   - [ ] Final failure moves job to dead-letter queue

#### Demo Script
```bash
# Start worker
python -m scheduler.worker --tags "cpu-heavy,default" --concurrency 4
# Worker worker-abc registered
# Listening for jobs...

# In another terminal, check workers
curl http://localhost:8000/api/workers
# Response: [{"id": "worker-abc", "tags": ["cpu-heavy", "default"], "active_jobs": 2}]

# Submit job and watch execution
curl -X POST http://localhost:8000/api/jobs/job-123/trigger
# Check job status
curl http://localhost:8000/api/runs/run-456
# Response: {"status": "running", "worker": "worker-abc", "started_at": "..."}

# Wait for completion
curl http://localhost:8000/api/runs/run-456
# Response: {"status": "completed", "duration": 45.2, "output": "Report generated"}

# Test exactly-once (trigger twice rapidly)
curl -X POST http://localhost:8000/api/jobs/job-123/trigger &
curl -X POST http://localhost:8000/api/jobs/job-123/trigger &
# Only one execution occurs
```

#### Test Requirements
- [ ] Test worker registration and heartbeat
- [ ] Test job assignment and locking
- [ ] Test timeout enforcement
- [ ] Test exactly-once with concurrent triggers
- [ ] Test retry with backoff
- [ ] Load test: 10,000 concurrent jobs

### Ticket 3: Monitoring Dashboard

**Summary:** Web UI for job status and history.

**Definition of Done:** Dashboard shows real-time job status.

#### Acceptance Criteria

1. **Job Overview**
   - [ ] List all jobs with status badges
   - [ ] Filter by: status, tag, schedule type
   - [ ] Search by job name
   - [ ] Pagination for large job lists

2. **Job Detail**
   - [ ] Show job configuration
   - [ ] Show recent runs (last 20)
   - [ ] Show success/failure rate
   - [ ] Show average duration

3. **Run History**
   - [ ] List all runs with status, duration, timestamp
   - [ ] Filter by: status, job, date range
   - [ ] Show run output (stdout/stderr)
   - [ ] Show retry attempts

4. **Real-time Updates**
   - [ ] WebSocket for live status updates
   - [ ] Running jobs show progress
   - [ ] Toast notifications for failures
   - [ ] Auto-refresh toggle

5. **Actions**
   - [ ] Trigger job manually
   - [ ] Cancel running job
   - [ ] Retry failed run
   - [ ] Disable/enable job

#### Demo Script
```bash
# Start dashboard
npm run dev --prefix dashboard
# Dashboard running at http://localhost:3000

# Open in browser
open http://localhost:3000

# Dashboard shows:
# - 150 jobs total, 3 running, 5 failed
# - Click job "daily-report" to see details
# - See chart of success rate over time
# - Click "Trigger Now" to run manually
# - See live log output while running

# Test WebSocket updates
# 1. Open dashboard in browser
# 2. Trigger job via API
# 3. See status change from "queued" -> "running" -> "completed" in real-time
```

#### Test Requirements
- [ ] End-to-end tests with Playwright
- [ ] Test job list filtering and search
- [ ] Test job detail page
- [ ] Test WebSocket updates
- [ ] Test manual trigger from UI
- [ ] Test responsive design (mobile)
