---
id: "test-011"
title: "Kubernetes Deployment Operator"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a Kubernetes operator that manages custom application deployments. It watches for custom resources and creates/updates the necessary Deployments, Services, and ConfigMaps. Written in Go using kubebuilder.

## Constraints

- Must be compatible with Kubernetes 1.28+
- Follow controller-runtime best practices

## Implementation Notes

- Use kubebuilder to scaffold the project
- Define CRD for Application resource
- Implement reconciliation logic
- Add status subresource for deployment state

## Review Notes

### Identified Weaknesses

1. **No RBAC definition**: What permissions does the operator need? Least privilege considerations?

2. **Missing finalizers**: How are child resources cleaned up when Application is deleted?

3. **No leader election**: Multiple operator replicas would conflict without it.

4. **Upgrade strategy undefined**: How do we handle CRD version upgrades and migrations?

5. **No admission webhooks**: Validation/mutation webhooks for better UX.

### Edge Cases

- What if a child resource is manually modified or deleted?
- How are partial failures handled (Deployment created but Service failed)?
- What about resources in terminating state?
- Namespace-scoped vs cluster-scoped operator?
- What if referenced ConfigMaps or Secrets don't exist?
- How are resource conflicts with existing objects handled?

### Assumptions to Validate

- Is kubebuilder the required framework, or would operator-sdk work?
- What fields should the Application CRD have?
- Is this operator deployed per-namespace or cluster-wide?
- Are there existing child resources to adopt, or greenfield only?
- What's the deployment target (managed Kubernetes, self-hosted, specific cloud)?

### Potential Failures

- Reconciliation loops (updates triggering more updates)
- Watch cache out of sync with etcd
- Rate limiting by Kubernetes API
- Resource quota exhaustion in target namespace
- Operator crash during critical multi-step operation
- CRD deletion while resources still exist

## Tickets

### Ticket 1: CRD Definition

**Summary:** Define the Application custom resource schema.

**Definition of Done:** CRD is applied and validates correctly.

### Ticket 2: Controller Logic

**Summary:** Implement reconciliation loop for Application resources.

**Definition of Done:** Controller creates child resources correctly.

### Ticket 3: Status Updates

**Summary:** Update Application status with deployment state.

**Definition of Done:** Status reflects actual state of deployment.
