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

(none yet)

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
