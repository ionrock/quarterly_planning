---
id: "test-011"
title: "Kubernetes Operator"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a Kubernetes operator that manages custom database instances. Handles provisioning, scaling, backups, and failover automatically.

## Constraints

- Recovery time under 60 seconds
- Support for PostgreSQL and MySQL

## Implementation Notes

- Use kubebuilder framework
- Custom Resource Definitions (CRDs)
- Reconciliation loop pattern

## Review Notes

(none yet)

## Tickets

### Ticket 1: CRD Definition

**Summary:** Define custom resource for database instances.

**Definition of Done:** CRD installed and instances creatable.

#### Steps

1. **Initialize kubebuilder project**
   - Run `kubebuilder init --domain example.com --repo github.com/example/db-operator`
   - Verify: project structure created

2. **Create Database API**
   - Run `kubebuilder create api --group database --version v1 --kind Database`
   - Answer yes to create resource and controller
   - Verify: API scaffolded

3. **Define DatabaseSpec struct**
   - Add Engine field (postgresql or mysql)
   - Add Version field (string)
   - Add Replicas field (int32)
   - Add Storage field (resource.Quantity)
   - Verify: spec compiles

4. **Define DatabaseStatus struct**
   - Add Phase field (Pending, Running, Failed)
   - Add Replicas field (current count)
   - Add Endpoint field (connection string)
   - Add LastBackup field (timestamp)
   - Verify: status compiles

5. **Add validation markers**
   - Add +kubebuilder:validation:Enum for Engine
   - Add +kubebuilder:validation:Minimum for Replicas
   - Add +kubebuilder:validation:Required for required fields
   - Verify: validation markers parsed

6. **Add printer columns**
   - Add +kubebuilder:printcolumn for Engine, Status, Replicas
   - Verify: kubectl get databases shows columns

7. **Generate CRD manifests**
   - Run `make manifests`
   - Verify: config/crd/bases/database.example.com_databases.yaml created

8. **Install CRD to cluster**
   - Run `make install`
   - Verify: `kubectl get crd databases.database.example.com` works

9. **Create sample Database resource**
   - Create config/samples/database_v1_database.yaml
   - Define sample PostgreSQL instance
   - Verify: `kubectl apply -f` succeeds

### Ticket 2: Reconciliation

**Summary:** Implement reconciliation loop for database state.

**Definition of Done:** Operator maintains desired state.

#### Steps

1. **Implement Reconcile function skeleton**
   - Fetch Database resource by name
   - Handle NotFound (deleted resource)
   - Verify: reconcile triggered on changes

2. **Add finalizer for cleanup**
   - Check if finalizer present
   - Add finalizer if missing
   - Handle deletion when finalizer present
   - Verify: cleanup runs on delete

3. **Create StatefulSet for database**
   - Generate StatefulSet spec matching Database spec
   - Set owner reference to Database
   - Verify: StatefulSet created with correct config

4. **Create Service for database**
   - Create headless Service for StatefulSet
   - Create ClusterIP Service for client access
   - Verify: Services created

5. **Create Secret for credentials**
   - Generate random password if not exists
   - Store in Secret with owner reference
   - Verify: Secret created with credentials

6. **Create ConfigMap for configuration**
   - Generate database config file
   - Store in ConfigMap
   - Mount in StatefulSet
   - Verify: ConfigMap mounted correctly

7. **Create PersistentVolumeClaims**
   - Use volumeClaimTemplates in StatefulSet
   - Set storage size from spec
   - Verify: PVCs created

8. **Update status on reconcile**
   - Check StatefulSet ready replicas
   - Update Database status accordingly
   - Set endpoint when ready
   - Verify: status reflects actual state

9. **Handle spec changes**
   - Detect changes to replicas, storage
   - Update StatefulSet to match
   - Verify: changes propagated

10. **Add event recording**
    - Record events for create, update, error
    - Verify: events visible via kubectl describe

### Ticket 3: Operations

**Summary:** Add backup, restore, and scaling operations.

**Definition of Done:** All operations work reliably.

#### Steps

1. **Create Backup CRD**
   - Run `kubebuilder create api --group database --version v1 --kind Backup`
   - Define spec with databaseRef, schedule (cron)
   - Verify: Backup CRD created

2. **Implement backup job creation**
   - Create Job that runs backup command
   - Use database credentials from Secret
   - Store backup to S3/GCS
   - Verify: backup job runs

3. **Track backup status**
   - Update Backup status with result
   - Update Database lastBackup timestamp
   - Verify: backup status tracked

4. **Create Restore CRD**
   - Define spec with databaseRef, backupRef
   - Verify: Restore CRD created

5. **Implement restore process**
   - Scale database to 0 replicas
   - Run restore job
   - Scale database back up
   - Verify: restore completes successfully

6. **Implement horizontal scaling**
   - Handle replicas increase
   - Add new replicas to StatefulSet
   - Configure replication
   - Verify: scaling up works

7. **Implement scale down**
   - Handle replicas decrease
   - Gracefully remove replicas
   - Verify: scaling down works

8. **Implement automatic failover**
   - Detect primary failure
   - Promote replica to primary
   - Update endpoint
   - Verify: failover completes < 60s

9. **Add health checks**
   - Create liveness probe for database process
   - Create readiness probe for query execution
   - Verify: unhealthy pods restarted

10. **Implement scheduled backups**
    - Create CronJob for backup schedule
    - Parse cron expression from spec
    - Verify: backups run on schedule

11. **Add metrics endpoint**
    - Export Prometheus metrics
    - Track reconcile duration, errors
    - Verify: metrics scrapable

12. **Write end-to-end tests**
    - Test create, scale, backup, restore, delete
    - Use envtest or kind cluster
    - Verify: e2e tests pass
