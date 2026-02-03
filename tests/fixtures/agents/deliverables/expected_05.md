---
id: "test-005"
title: "Kubernetes Operator for PostgreSQL"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a Kubernetes operator that manages PostgreSQL clusters. Handles provisioning, backups, failover, and scaling. Written in Go using controller-runtime.

## Constraints

- Must support PostgreSQL 14, 15, and 16
- Recovery time objective (RTO) under 30 seconds

## Implementation Notes

- Kubebuilder for scaffolding
- Patroni for HA management
- pgBackRest for backups

## Review Notes

(none yet)

## Tickets

### Ticket 1: Custom Resource Definition

**Summary:** Define PostgresCluster CRD with validation.

**Definition of Done:** CRD installed and validated by Kubernetes.

#### Acceptance Criteria

1. **CRD Schema**
   - [ ] apiVersion: postgres.example.com/v1
   - [ ] kind: PostgresCluster
   - [ ] spec.version: enum [14, 15, 16]
   - [ ] spec.replicas: integer, min 1, max 10
   - [ ] spec.storage.size: string (e.g., "10Gi")
   - [ ] spec.storage.storageClass: string
   - [ ] spec.resources.requests/limits for CPU and memory

2. **Validation**
   - [ ] Required fields: version, replicas, storage.size
   - [ ] Version must be 14, 15, or 16
   - [ ] Replicas minimum 1, maximum 10
   - [ ] Storage size parseable as Kubernetes quantity
   - [ ] Invalid specs rejected with clear error message

3. **Status Subresource**
   - [ ] status.phase: Pending, Creating, Running, Failed
   - [ ] status.readyReplicas: integer
   - [ ] status.primaryPod: string (pod name)
   - [ ] status.conditions: array of Condition objects

4. **Additional Printer Columns**
   - [ ] VERSION column shows spec.version
   - [ ] REPLICAS shows status.readyReplicas/spec.replicas
   - [ ] STATUS shows status.phase
   - [ ] AGE shows creation timestamp

#### Demo Script
```bash
# Install CRD
kubectl apply -f config/crd/bases/postgres.example.com_postgresclusters.yaml

# Verify CRD installed
kubectl get crd postgresclusters.postgres.example.com
# Expected: NAME ... CREATED AT

# Create valid cluster
kubectl apply -f - <<EOF
apiVersion: postgres.example.com/v1
kind: PostgresCluster
metadata:
  name: my-cluster
spec:
  version: 16
  replicas: 3
  storage:
    size: 10Gi
EOF
# Expected: postgrescluster.postgres.example.com/my-cluster created

# Try invalid version
kubectl apply -f - <<EOF
apiVersion: postgres.example.com/v1
kind: PostgresCluster
metadata:
  name: bad-cluster
spec:
  version: 13
  replicas: 1
  storage:
    size: 10Gi
EOF
# Expected: Error: spec.version must be one of [14, 15, 16]

# Check printer columns
kubectl get postgrescluster
# NAME         VERSION   REPLICAS   STATUS    AGE
# my-cluster   16        0/3        Pending   5s
```

#### Test Requirements
- [ ] Unit tests for validation webhooks
- [ ] Integration test: CRD creation in kind cluster
- [ ] Test all invalid input combinations
- [ ] Test status updates reflect correctly

### Ticket 2: Cluster Provisioning

**Summary:** Reconcile loop creates StatefulSet, Services, ConfigMaps.

**Definition of Done:** PostgreSQL cluster runs with primary and replicas.

#### Acceptance Criteria

1. **StatefulSet Creation**
   - [ ] StatefulSet created with spec.replicas pods
   - [ ] Pod template includes PostgreSQL and Patroni containers
   - [ ] PVC template matches spec.storage
   - [ ] Pod anti-affinity for HA (prefer different nodes)

2. **Service Creation**
   - [ ] Primary service (read-write): {name}-primary
   - [ ] Replica service (read-only): {name}-replica
   - [ ] Headless service for StatefulSet: {name}-pods
   - [ ] Services have correct selectors

3. **ConfigMaps and Secrets**
   - [ ] ConfigMap with postgresql.conf settings
   - [ ] ConfigMap with Patroni configuration
   - [ ] Secret with superuser password (generated)
   - [ ] Secret with replication user password

4. **Reconciliation**
   - [ ] Changes to spec.replicas scale StatefulSet
   - [ ] Changes to spec.resources update pod template
   - [ ] Reconcile retries on transient errors
   - [ ] Status updated after each reconcile

5. **Health Checks**
   - [ ] Liveness probe: pg_isready
   - [ ] Readiness probe: Patroni API /health
   - [ ] Startup probe with 60s timeout

#### Demo Script
```bash
# Create cluster
kubectl apply -f examples/basic-cluster.yaml

# Watch provisioning
kubectl get postgrescluster -w
# NAME         VERSION   REPLICAS   STATUS     AGE
# my-cluster   16        0/3        Creating   5s
# my-cluster   16        1/3        Creating   30s
# my-cluster   16        3/3        Running    90s

# Verify resources created
kubectl get statefulset,svc,configmap,secret -l postgres.example.com/cluster=my-cluster
# Expected: StatefulSet, 3 Services, 2 ConfigMaps, 2 Secrets

# Connect to primary
kubectl run -it --rm psql --image=postgres:16 --restart=Never -- \
  psql -h my-cluster-primary -U postgres
# Expected: PostgreSQL prompt

# Scale replicas
kubectl patch postgrescluster my-cluster --type=merge -p '{"spec":{"replicas":5}}'
kubectl get pods -w
# Expected: 2 new pods created
```

#### Test Requirements
- [ ] Integration test in kind cluster
- [ ] Test scale up from 1 to 5 replicas
- [ ] Test scale down from 5 to 2 replicas
- [ ] Test primary failover (delete primary pod)
- [ ] Verify failover completes in <30 seconds

### Ticket 3: Backup Management

**Summary:** Scheduled backups with retention policy.

**Definition of Done:** Backups run on schedule and old ones are cleaned up.

#### Acceptance Criteria

1. **Backup CRD**
   - [ ] kind: PostgresBackup
   - [ ] spec.clusterName: reference to PostgresCluster
   - [ ] spec.schedule: cron expression (optional for one-time)
   - [ ] spec.retention.days: integer (default 7)
   - [ ] spec.retention.count: integer (default 5)

2. **Scheduled Backups**
   - [ ] CronJob created for scheduled backups
   - [ ] Backup job runs pgBackRest
   - [ ] Job pods cleaned up after completion
   - [ ] Failed backups retry 3 times

3. **Backup Execution**
   - [ ] Full backup taken on first run
   - [ ] Incremental backups on subsequent runs
   - [ ] Backup stored to configured location (PVC or S3)
   - [ ] Backup metadata recorded in status

4. **Retention Policy**
   - [ ] Backups older than retention.days deleted
   - [ ] At most retention.count backups kept
   - [ ] Cleanup runs after each successful backup
   - [ ] Manual backups can be marked as "keep forever"

5. **Status Reporting**
   - [ ] status.lastBackup: timestamp
   - [ ] status.lastBackupSize: string (e.g., "1.2GB")
   - [ ] status.backupCount: integer
   - [ ] status.nextScheduled: timestamp

#### Demo Script
```bash
# Create backup schedule
kubectl apply -f - <<EOF
apiVersion: postgres.example.com/v1
kind: PostgresBackup
metadata:
  name: my-cluster-backup
spec:
  clusterName: my-cluster
  schedule: "0 2 * * *"  # 2 AM daily
  retention:
    days: 14
    count: 10
EOF

# Trigger manual backup
kubectl annotate postgresbackup my-cluster-backup backup.postgres.example.com/trigger=now

# Watch backup progress
kubectl get postgresbackup -w
# NAME                CLUSTER      LAST BACKUP           SIZE    COUNT   NEXT SCHEDULED
# my-cluster-backup   my-cluster   2024-01-15T02:00:00   1.2GB   5       2024-01-16T02:00:00

# List backups
kubectl exec my-cluster-0 -- pgbackrest info
# Expected: List of backups with timestamps and sizes

# Restore from backup (creates new cluster)
kubectl apply -f - <<EOF
apiVersion: postgres.example.com/v1
kind: PostgresCluster
metadata:
  name: restored-cluster
spec:
  version: 16
  replicas: 1
  storage:
    size: 10Gi
  restore:
    backupName: my-cluster-backup
    targetTime: "2024-01-15T01:30:00Z"  # Point-in-time recovery
EOF
```

#### Test Requirements
- [ ] Integration test for backup execution
- [ ] Test scheduled backup (fast-forward time)
- [ ] Test retention cleanup
- [ ] Test restore to new cluster
- [ ] Test point-in-time recovery
- [ ] Test backup with S3 storage backend
