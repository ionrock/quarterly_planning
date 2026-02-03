---
id: "test-006"
title: "OAuth2 Authentication Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Implement an OAuth2 authentication service that supports multiple providers (Google, GitHub, Microsoft). Handles token exchange, user creation, and session management.

## Constraints

- Tokens must be securely stored
- Support refresh token rotation

## Implementation Notes

- Use passport.js for OAuth strategies
- JWT for session tokens
- PostgreSQL for user storage

## Review Notes

(none yet)

## Tickets

### Ticket 1: OAuth Provider Setup

**Summary:** Configure OAuth providers and callbacks.

**Definition of Done:** Can initiate OAuth flow with all providers.

#### Steps

1. **Install passport dependencies**
   - Run `npm install passport passport-google-oauth20 passport-github2 passport-microsoft`
   - Verify: packages in package.json

2. **Create passport configuration module**
   - Create src/auth/passport.ts
   - Initialize passport
   - Verify: passport initializes without error

3. **Configure Google OAuth strategy**
   - Add GoogleStrategy with clientID, clientSecret, callbackURL
   - Read credentials from GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET env vars
   - Verify: strategy registered with passport

4. **Configure GitHub OAuth strategy**
   - Add GitHubStrategy with clientID, clientSecret, callbackURL
   - Read credentials from GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET env vars
   - Verify: strategy registered with passport

5. **Configure Microsoft OAuth strategy**
   - Add MicrosoftStrategy with clientID, clientSecret, callbackURL
   - Read credentials from MICROSOFT_CLIENT_ID, MICROSOFT_CLIENT_SECRET env vars
   - Verify: strategy registered with passport

6. **Create OAuth initiation routes**
   - GET /auth/google -> passport.authenticate('google', { scope: ['profile', 'email'] })
   - GET /auth/github -> passport.authenticate('github', { scope: ['user:email'] })
   - GET /auth/microsoft -> passport.authenticate('microsoft', { scope: ['user.read'] })
   - Verify: routes redirect to provider login

7. **Create OAuth callback routes**
   - GET /auth/google/callback -> handle Google response
   - GET /auth/github/callback -> handle GitHub response
   - GET /auth/microsoft/callback -> handle Microsoft response
   - Verify: callbacks receive user profile

8. **Add state parameter for CSRF protection**
   - Generate random state before redirect
   - Verify state matches on callback
   - Verify: mismatched state rejected

9. **Handle OAuth errors**
   - Catch denied access, invalid grant errors
   - Redirect to frontend with error message
   - Verify: errors handled gracefully

### Ticket 2: Token Management

**Summary:** Handle access and refresh tokens.

**Definition of Done:** Tokens are issued, validated, and refreshed.

#### Steps

1. **Install JWT dependencies**
   - Run `npm install jsonwebtoken`
   - Add @types/jsonwebtoken for TypeScript
   - Verify: packages installed

2. **Create JWT configuration**
   - Create src/auth/jwt.ts
   - Read JWT_SECRET from environment (require minimum 32 chars)
   - Set access token expiry to 15 minutes
   - Set refresh token expiry to 7 days
   - Verify: config loads correctly

3. **Implement access token generation**
   - Create generateAccessToken(userId, email): string
   - Include userId, email, type: 'access' in payload
   - Sign with JWT_SECRET and expiry
   - Verify: generates valid JWT

4. **Implement refresh token generation**
   - Create generateRefreshToken(userId): string
   - Include userId, type: 'refresh', jti (unique ID) in payload
   - Sign with JWT_SECRET and expiry
   - Verify: generates valid JWT with unique jti

5. **Create refresh tokens table**
   - Columns: id, user_id, jti, expires_at, revoked_at, created_at
   - Index on user_id and jti
   - Verify: migration runs

6. **Store refresh token on generation**
   - Insert record with jti and expiry
   - Link to user_id
   - Verify: token record created in database

7. **Implement token validation middleware**
   - Create src/middleware/authenticate.ts
   - Extract Bearer token from Authorization header
   - Verify JWT signature and expiry
   - Attach user to request object
   - Verify: valid tokens pass, invalid rejected

8. **Implement refresh token endpoint**
   - POST /auth/refresh accepts { refreshToken }
   - Verify refresh token validity and not revoked
   - Generate new access token and refresh token (rotation)
   - Revoke old refresh token
   - Verify: new tokens issued, old token unusable

9. **Implement token revocation**
   - POST /auth/revoke accepts { refreshToken }
   - Mark refresh token as revoked in database
   - Verify: revoked token rejected on refresh

10. **Add logout endpoint**
    - POST /auth/logout revokes current refresh token
    - Clear client-side tokens (return instruction)
    - Verify: logout invalidates session

### Ticket 3: User Management

**Summary:** Create and link user accounts.

**Definition of Done:** Users created on first login, linked on subsequent.

#### Steps

1. **Create users table**
   - Columns: id, email, name, avatar_url, created_at, updated_at
   - Unique constraint on email
   - Verify: migration runs

2. **Create oauth_accounts table**
   - Columns: id, user_id, provider, provider_user_id, created_at
   - Unique constraint on (provider, provider_user_id)
   - Foreign key to users
   - Verify: migration runs

3. **Create User model**
   - Create src/models/user.ts
   - Define User interface
   - Verify: interface compiles

4. **Create user repository**
   - Create src/repositories/userRepository.ts
   - Implement findById, findByEmail, create, update
   - Verify: CRUD operations work

5. **Create oauth account repository**
   - Create src/repositories/oauthAccountRepository.ts
   - Implement findByProvider, create, linkToUser
   - Verify: operations work

6. **Implement user creation on first OAuth login**
   - In OAuth callback, check if provider account exists
   - If not, create new user with profile data
   - Create oauth_account linking provider to user
   - Verify: new user created on first Google login

7. **Implement account linking for existing users**
   - If provider account doesn't exist but email matches existing user
   - Link OAuth account to existing user
   - Verify: GitHub login links to existing Google user (same email)

8. **Handle email conflicts**
   - If email exists but different OAuth provider
   - Prompt user to link accounts or use existing provider
   - Verify: conflict handled gracefully

9. **Update user profile on login**
   - Update name, avatar_url if changed in provider
   - Verify: profile changes reflected

10. **Add profile endpoint**
    - GET /auth/me returns current user profile
    - Include linked OAuth providers
    - Verify: returns user data for authenticated request

11. **Implement account unlinking**
    - DELETE /auth/providers/:provider unlinks OAuth account
    - Prevent unlinking last authentication method
    - Verify: can unlink, cannot unlink last
