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

### Ticket 2: Cluster Provisioning

**Summary:** Reconcile loop creates StatefulSet, Services, ConfigMaps.

**Definition of Done:** PostgreSQL cluster runs with primary and replicas.

### Ticket 3: Backup Management

**Summary:** Scheduled backups with retention policy.

**Definition of Done:** Backups run on schedule and old ones are cleaned up.
