---
id: "test-009"
title: "OAuth2 Authorization Server"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build an OAuth2 authorization server implementing RFC 6749. Supports authorization code, client credentials, and refresh token grants. Includes user consent UI.

## Constraints

- Must pass OAuth2 conformance tests
- Token generation under 50ms

## Implementation Notes

- Node.js with Express
- PostgreSQL for persistence
- JWT for access tokens

## Review Notes

(none yet)

## Tickets

### Ticket 1: Authorization Code Flow

**Summary:** Implement authorization code grant with PKCE.

**Definition of Done:** Code flow works with standard OAuth2 clients.

#### Acceptance Criteria

1. **Authorization Endpoint**
   - [ ] GET /oauth/authorize initiates flow
   - [ ] Required params: response_type=code, client_id, redirect_uri
   - [ ] Optional params: scope, state, code_challenge, code_challenge_method
   - [ ] Invalid client_id returns error page (not redirect)
   - [ ] Invalid redirect_uri returns error page (not redirect)

2. **PKCE Support (RFC 7636)**
   - [ ] code_challenge_method: S256 (required) or plain
   - [ ] code_challenge stored with auth code
   - [ ] code_verifier validated at token endpoint
   - [ ] PKCE required for public clients

3. **Authorization Code**
   - [ ] Code generated after user consent
   - [ ] Code is single-use
   - [ ] Code expires in 10 minutes
   - [ ] Code bound to client_id and redirect_uri

4. **Redirect Handling**
   - [ ] Success: redirect with code and state
   - [ ] Deny: redirect with error=access_denied
   - [ ] Redirect URI must match registered URI exactly
   - [ ] Fragment (#) not allowed in redirect URI

5. **Scope Handling**
   - [ ] Requested scopes validated against client's allowed scopes
   - [ ] Unknown scope returns error
   - [ ] Granted scopes may be subset of requested

#### Demo Script
```bash
# Start authorization (browser)
open "http://localhost:3000/oauth/authorize?\
response_type=code&\
client_id=my-app&\
redirect_uri=http://localhost:8080/callback&\
scope=read%20write&\
state=xyz123&\
code_challenge=E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM&\
code_challenge_method=S256"

# After user login and consent, redirects to:
# http://localhost:8080/callback?code=AUTH_CODE&state=xyz123

# Exchange code for token
curl -X POST http://localhost:3000/oauth/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=authorization_code&\
code=AUTH_CODE&\
redirect_uri=http://localhost:8080/callback&\
client_id=my-app&\
code_verifier=dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"

# Response:
# {"access_token": "eyJ...", "token_type": "Bearer", "expires_in": 3600, "refresh_token": "..."}
```

#### Test Requirements
- [ ] Test complete authorization code flow
- [ ] Test PKCE validation
- [ ] Test code single-use
- [ ] Test code expiration
- [ ] Test invalid redirect_uri handling
- [ ] Run OAuth2 conformance test suite

### Ticket 2: Token Management

**Summary:** Issue, validate, and revoke tokens.

**Definition of Done:** All token operations work correctly.

#### Acceptance Criteria

1. **Access Token Issuance**
   - [ ] JWT format with standard claims
   - [ ] Claims: iss, sub, aud, exp, iat, jti, scope
   - [ ] Signed with RS256 (RSA) or ES256 (ECDSA)
   - [ ] Expiration configurable per client (default 1 hour)
   - [ ] JWKS endpoint exposes public key

2. **Refresh Token**
   - [ ] Opaque token (not JWT)
   - [ ] Stored in database with metadata
   - [ ] Longer expiration (30 days default)
   - [ ] Rotated on use (one-time use)
   - [ ] Refresh issues new access + refresh tokens

3. **Token Validation**
   - [ ] POST /oauth/introspect validates token
   - [ ] Returns active=true/false and token metadata
   - [ ] Supports both access and refresh tokens
   - [ ] Client authentication required

4. **Token Revocation**
   - [ ] POST /oauth/revoke revokes token
   - [ ] Revokes refresh token and all access tokens from it
   - [ ] Returns 200 even if token already revoked
   - [ ] Supports token_type_hint parameter

5. **Client Credentials Grant**
   - [ ] POST /oauth/token with grant_type=client_credentials
   - [ ] Client authenticates with client_id and client_secret
   - [ ] No refresh token issued (just access token)
   - [ ] Scope limited to client's service scopes

#### Demo Script
```bash
# Token introspection
curl -X POST http://localhost:3000/oauth/introspect \
  -u "my-app:client-secret" \
  -d "token=eyJ..."
# Response: {"active": true, "sub": "user-123", "scope": "read write", ...}

# Refresh token
curl -X POST http://localhost:3000/oauth/token \
  -d "grant_type=refresh_token&\
refresh_token=REFRESH_TOKEN&\
client_id=my-app&\
client_secret=client-secret"
# Response: {"access_token": "new...", "refresh_token": "new...", ...}

# Revoke token
curl -X POST http://localhost:3000/oauth/revoke \
  -u "my-app:client-secret" \
  -d "token=eyJ..."
# Response: 200 OK

# Client credentials (service-to-service)
curl -X POST http://localhost:3000/oauth/token \
  -u "service-client:service-secret" \
  -d "grant_type=client_credentials&scope=api.read"
# Response: {"access_token": "eyJ...", "token_type": "Bearer", "expires_in": 3600}

# JWKS endpoint
curl http://localhost:3000/.well-known/jwks.json
# Response: {"keys": [{"kty": "RSA", "kid": "key-1", ...}]}
```

#### Test Requirements
- [ ] Test JWT generation and validation
- [ ] Test refresh token rotation
- [ ] Test token revocation
- [ ] Test introspection
- [ ] Test client credentials grant
- [ ] Benchmark: 10,000 token generations/second

### Ticket 3: Consent UI

**Summary:** User-facing consent and login pages.

**Definition of Done:** Users can authorize or deny access requests.

#### Acceptance Criteria

1. **Login Page**
   - [ ] Username/password form
   - [ ] Error message for invalid credentials
   - [ ] Rate limiting (5 attempts per minute)
   - [ ] "Remember me" option
   - [ ] Session cookie set on success

2. **Consent Page**
   - [ ] Shows client name and logo
   - [ ] Lists requested scopes with descriptions
   - [ ] "Allow" and "Deny" buttons
   - [ ] Shows previously granted scopes
   - [ ] Option to modify granted scopes

3. **Skip Consent**
   - [ ] Skip for previously authorized scopes
   - [ ] Skip configurable per client (trusted clients)
   - [ ] Force consent with prompt=consent

4. **Error Pages**
   - [ ] Invalid client error page
   - [ ] Invalid redirect URI error page
   - [ ] Generic error page with error code

5. **Security**
   - [ ] CSRF protection on all forms
   - [ ] CSP headers preventing XSS
   - [ ] Login page not frameable (X-Frame-Options)
   - [ ] Secure session cookies (HttpOnly, Secure, SameSite)

#### Demo Script
```bash
# Navigate to authorization URL in browser
open "http://localhost:3000/oauth/authorize?..."

# 1. If not logged in:
#    - See login form
#    - Enter credentials
#    - Submit form

# 2. If logged in but new scopes:
#    - See consent page
#    - Client "My App" is requesting:
#      - [x] Read your profile
#      - [x] Access your email
#    - Click "Allow" or "Deny"

# 3. If already authorized with same scopes:
#    - Redirect immediately to callback

# Test CSRF
curl -X POST http://localhost:3000/oauth/consent \
  -d "allow=true&client_id=my-app"
# Response: 403 Forbidden (missing CSRF token)
```

#### Test Requirements
- [ ] End-to-end test of login flow
- [ ] End-to-end test of consent flow
- [ ] Test "deny" flow
- [ ] Test skip consent behavior
- [ ] Test CSRF protection
- [ ] Test rate limiting on login
- [ ] Accessibility audit (WCAG 2.1 AA)
