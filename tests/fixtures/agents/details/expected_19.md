---
id: "test-019"
title: "S3-Compatible Object Storage"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build an S3-compatible object storage server for development and testing. Supports basic S3 operations (GET, PUT, DELETE, LIST) with bucket management. Stores objects on local filesystem. Written in Go.

## Constraints

- Compatible with AWS SDK
- Support objects up to 5GB

## Implementation Notes

### Technology Stack
- **Language:** Go 1.21+
- **Router:** chi for HTTP routing
- **Storage:** Local filesystem with SQLite metadata
- **Auth:** AWS Signature V4 verification

### S3 API Endpoints
```
PUT    /{bucket}                   - Create bucket
DELETE /{bucket}                   - Delete bucket
GET    /{bucket}                   - List objects
HEAD   /{bucket}                   - Bucket exists check

PUT    /{bucket}/{key...}          - Put object
GET    /{bucket}/{key...}          - Get object
DELETE /{bucket}/{key...}          - Delete object
HEAD   /{bucket}/{key...}          - Object metadata

POST   /{bucket}/{key}?uploads     - Initiate multipart upload
PUT    /{bucket}/{key}?partNumber  - Upload part
POST   /{bucket}/{key}?uploadId    - Complete multipart upload
```

### Data Structures
```go
type Bucket struct {
    Name      string    `json:"name"`
    CreatedAt time.Time `json:"createdAt"`
}

type Object struct {
    Key          string            `json:"key"`
    Size         int64             `json:"size"`
    ETag         string            `json:"etag"`
    ContentType  string            `json:"contentType"`
    Metadata     map[string]string `json:"metadata"`
    LastModified time.Time         `json:"lastModified"`
}

type ListObjectsResponse struct {
    Name           string   `xml:"Name"`
    Prefix         string   `xml:"Prefix"`
    MaxKeys        int      `xml:"MaxKeys"`
    IsTruncated    bool     `xml:"IsTruncated"`
    Contents       []Object `xml:"Contents"`
    NextMarker     string   `xml:"NextContinuationToken,omitempty"`
}
```

### Storage Layout
```
data/
├── buckets/
│   ├── my-bucket/
│   │   ├── objects/
│   │   │   ├── ab/
│   │   │   │   └── abcdef123456  (content-addressed)
│   │   │   └── cd/
│   │   │       └── cdef789012
│   │   └── multipart/
│   │       └── upload-id/
│   │           ├── part.1
│   │           └── part.2
│   └── another-bucket/
└── metadata.db  (SQLite)
```

### SQLite Schema
```sql
CREATE TABLE buckets (
    name TEXT PRIMARY KEY,
    created_at DATETIME NOT NULL
);

CREATE TABLE objects (
    bucket TEXT NOT NULL,
    key TEXT NOT NULL,
    size INTEGER NOT NULL,
    etag TEXT NOT NULL,
    content_type TEXT,
    storage_path TEXT NOT NULL,
    metadata TEXT,  -- JSON
    created_at DATETIME NOT NULL,
    PRIMARY KEY (bucket, key),
    FOREIGN KEY (bucket) REFERENCES buckets(name)
);

CREATE TABLE multipart_uploads (
    upload_id TEXT PRIMARY KEY,
    bucket TEXT NOT NULL,
    key TEXT NOT NULL,
    created_at DATETIME NOT NULL
);

CREATE TABLE multipart_parts (
    upload_id TEXT NOT NULL,
    part_number INTEGER NOT NULL,
    size INTEGER NOT NULL,
    etag TEXT NOT NULL,
    storage_path TEXT NOT NULL,
    PRIMARY KEY (upload_id, part_number)
);
```

### Put Object Handler
```go
func (s *Server) PutObject(w http.ResponseWriter, r *http.Request) {
    bucket := chi.URLParam(r, "bucket")
    key := chi.URLParam(r, "*")

    // Verify bucket exists
    if !s.bucketExists(bucket) {
        s.errorResponse(w, "NoSuchBucket", http.StatusNotFound)
        return
    }

    // Read body with size limit
    body := http.MaxBytesReader(w, r.Body, s.maxObjectSize)

    // Calculate hash while reading
    hash := md5.New()
    tempFile, err := os.CreateTemp(s.tempDir, "upload-*")
    if err != nil {
        s.errorResponse(w, "InternalError", http.StatusInternalServerError)
        return
    }
    defer os.Remove(tempFile.Name())

    size, err := io.Copy(io.MultiWriter(tempFile, hash), body)
    if err != nil {
        s.errorResponse(w, "InternalError", http.StatusInternalServerError)
        return
    }

    etag := fmt.Sprintf("\"%x\"", hash.Sum(nil))
    storagePath := s.objectPath(bucket, etag)

    // Move to final location
    os.MkdirAll(filepath.Dir(storagePath), 0755)
    if err := os.Rename(tempFile.Name(), storagePath); err != nil {
        s.errorResponse(w, "InternalError", http.StatusInternalServerError)
        return
    }

    // Update metadata
    obj := Object{
        Key:          key,
        Size:         size,
        ETag:         etag,
        ContentType:  r.Header.Get("Content-Type"),
        Metadata:     extractUserMetadata(r.Header),
        LastModified: time.Now(),
    }
    s.db.UpsertObject(bucket, obj, storagePath)

    w.Header().Set("ETag", etag)
    w.WriteHeader(http.StatusOK)
}
```

### AWS Signature V4 Verification
```go
func (s *Server) verifySignature(r *http.Request) error {
    authHeader := r.Header.Get("Authorization")
    if authHeader == "" {
        return ErrMissingAuth
    }

    // Parse: AWS4-HMAC-SHA256 Credential=.../..., SignedHeaders=..., Signature=...
    parts := parseAuthHeader(authHeader)

    // Recreate canonical request
    canonicalRequest := fmt.Sprintf("%s\n%s\n%s\n%s\n%s\n%s",
        r.Method,
        canonicalURI(r.URL.Path),
        canonicalQueryString(r.URL.RawQuery),
        canonicalHeaders(r.Header, parts.SignedHeaders),
        parts.SignedHeaders,
        r.Header.Get("X-Amz-Content-Sha256"),
    )

    // Create string to sign
    stringToSign := fmt.Sprintf("AWS4-HMAC-SHA256\n%s\n%s\n%s",
        r.Header.Get("X-Amz-Date"),
        parts.CredentialScope,
        sha256Hex(canonicalRequest),
    )

    // Calculate expected signature
    signingKey := deriveSigningKey(s.secretKey, parts.Date, parts.Region, "s3")
    expectedSig := hmacSHA256Hex(signingKey, stringToSign)

    if !hmac.Equal([]byte(expectedSig), []byte(parts.Signature)) {
        return ErrInvalidSignature
    }

    return nil
}
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: Basic Operations

**Summary:** Implement GET, PUT, DELETE for objects.

**Definition of Done:** Basic object operations work with AWS CLI.

### Ticket 2: Bucket Management

**Summary:** Implement bucket create, delete, and list operations.

**Definition of Done:** Buckets can be managed via API.

### Ticket 3: Multipart Upload

**Summary:** Implement multipart upload for large files.

**Definition of Done:** Files over 5MB can be uploaded in parts.
