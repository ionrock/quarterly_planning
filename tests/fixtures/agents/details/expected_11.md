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

### Technology Stack
- **Framework:** Kubebuilder 3.x
- **Controller Runtime:** controller-runtime v0.16+
- **Client:** client-go for Kubernetes API
- **Testing:** envtest for integration tests

### Custom Resource Definition
```yaml
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: applications.apps.example.com
spec:
  group: apps.example.com
  versions:
    - name: v1alpha1
      served: true
      storage: true
      schema:
        openAPIV3Schema:
          type: object
          properties:
            spec:
              type: object
              required: [image, replicas]
              properties:
                image:
                  type: string
                replicas:
                  type: integer
                  minimum: 1
                  maximum: 100
                port:
                  type: integer
                  default: 8080
                env:
                  type: array
                  items:
                    type: object
                    properties:
                      name: { type: string }
                      value: { type: string }
                resources:
                  type: object
                  properties:
                    cpu: { type: string, default: "100m" }
                    memory: { type: string, default: "128Mi" }
            status:
              type: object
              properties:
                phase: { type: string }
                availableReplicas: { type: integer }
                conditions:
                  type: array
                  items:
                    type: object
                    properties:
                      type: { type: string }
                      status: { type: string }
                      lastTransitionTime: { type: string }
                      reason: { type: string }
                      message: { type: string }
      subresources:
        status: {}
  scope: Namespaced
  names:
    plural: applications
    singular: application
    kind: Application
    shortNames: [app]
```

### Go Types
```go
type ApplicationSpec struct {
    Image     string            `json:"image"`
    Replicas  int32             `json:"replicas"`
    Port      int32             `json:"port,omitempty"`
    Env       []EnvVar          `json:"env,omitempty"`
    Resources ResourceRequirements `json:"resources,omitempty"`
}

type ApplicationStatus struct {
    Phase             string             `json:"phase,omitempty"`
    AvailableReplicas int32              `json:"availableReplicas,omitempty"`
    Conditions        []metav1.Condition `json:"conditions,omitempty"`
}

// +kubebuilder:object:root=true
// +kubebuilder:subresource:status
// +kubebuilder:printcolumn:name="Replicas",type=integer,JSONPath=`.spec.replicas`
// +kubebuilder:printcolumn:name="Available",type=integer,JSONPath=`.status.availableReplicas`
// +kubebuilder:printcolumn:name="Phase",type=string,JSONPath=`.status.phase`
type Application struct {
    metav1.TypeMeta   `json:",inline"`
    metav1.ObjectMeta `json:"metadata,omitempty"`
    Spec              ApplicationSpec   `json:"spec,omitempty"`
    Status            ApplicationStatus `json:"status,omitempty"`
}
```

### Reconciler Implementation
```go
func (r *ApplicationReconciler) Reconcile(ctx context.Context, req ctrl.Request) (ctrl.Result, error) {
    log := log.FromContext(ctx)

    // Fetch the Application
    var app appsv1alpha1.Application
    if err := r.Get(ctx, req.NamespacedName, &app); err != nil {
        if apierrors.IsNotFound(err) {
            return ctrl.Result{}, nil
        }
        return ctrl.Result{}, err
    }

    // Handle deletion with finalizer
    if !app.DeletionTimestamp.IsZero() {
        return r.reconcileDelete(ctx, &app)
    }

    // Add finalizer if not present
    if !controllerutil.ContainsFinalizer(&app, finalizerName) {
        controllerutil.AddFinalizer(&app, finalizerName)
        if err := r.Update(ctx, &app); err != nil {
            return ctrl.Result{}, err
        }
    }

    // Reconcile child resources
    if err := r.reconcileDeployment(ctx, &app); err != nil {
        return ctrl.Result{}, err
    }
    if err := r.reconcileService(ctx, &app); err != nil {
        return ctrl.Result{}, err
    }

    // Update status
    if err := r.updateStatus(ctx, &app); err != nil {
        return ctrl.Result{}, err
    }

    return ctrl.Result{RequeueAfter: 30 * time.Second}, nil
}
```

### Child Resource Creation
```go
func (r *ApplicationReconciler) reconcileDeployment(ctx context.Context, app *appsv1alpha1.Application) error {
    deploy := &appsv1.Deployment{
        ObjectMeta: metav1.ObjectMeta{
            Name:      app.Name,
            Namespace: app.Namespace,
        },
    }

    op, err := controllerutil.CreateOrUpdate(ctx, r.Client, deploy, func() error {
        deploy.Spec = appsv1.DeploymentSpec{
            Replicas: &app.Spec.Replicas,
            Selector: &metav1.LabelSelector{
                MatchLabels: map[string]string{"app": app.Name},
            },
            Template: corev1.PodTemplateSpec{
                ObjectMeta: metav1.ObjectMeta{
                    Labels: map[string]string{"app": app.Name},
                },
                Spec: corev1.PodSpec{
                    Containers: []corev1.Container{{
                        Name:  "app",
                        Image: app.Spec.Image,
                        Ports: []corev1.ContainerPort{{
                            ContainerPort: app.Spec.Port,
                        }},
                    }},
                },
            },
        }
        return controllerutil.SetControllerReference(app, deploy, r.Scheme)
    })

    if err != nil {
        return fmt.Errorf("create/update deployment: %w", err)
    }
    log.Info("Deployment reconciled", "operation", op)
    return nil
}
```

### RBAC Configuration
```go
// +kubebuilder:rbac:groups=apps.example.com,resources=applications,verbs=get;list;watch;create;update;patch;delete
// +kubebuilder:rbac:groups=apps.example.com,resources=applications/status,verbs=get;update;patch
// +kubebuilder:rbac:groups=apps,resources=deployments,verbs=get;list;watch;create;update;patch;delete
// +kubebuilder:rbac:groups=core,resources=services,verbs=get;list;watch;create;update;patch;delete
// +kubebuilder:rbac:groups=core,resources=configmaps,verbs=get;list;watch;create;update;patch;delete
```

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
