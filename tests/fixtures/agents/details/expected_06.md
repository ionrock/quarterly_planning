---
id: "test-006"
title: "OAuth2 Authentication Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Implement an OAuth2 authentication service that supports Google and GitHub login. Users can authenticate via third-party providers and receive JWT tokens for API access. Built with Python/FastAPI.

## Constraints

- Must comply with OAuth2 specification
- Tokens expire after 1 hour

## Implementation Notes

### Technology Stack
- **Framework:** FastAPI 0.100+
- **OAuth Library:** Authlib 1.x
- **JWT:** PyJWT with RS256 signing
- **Database:** PostgreSQL with asyncpg + SQLAlchemy 2.0
- **Password Hashing:** argon2-cffi (for future local auth)

### OAuth2 Flow Endpoints
```
GET  /auth/google          - Redirect to Google OAuth
GET  /auth/google/callback - Handle Google callback
GET  /auth/github          - Redirect to GitHub OAuth
GET  /auth/github/callback - Handle GitHub callback
POST /auth/refresh         - Refresh access token
POST /auth/logout          - Revoke refresh token
GET  /auth/me              - Get current user info
```

### Database Schema
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255),
    avatar_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ
);

CREATE TABLE oauth_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL,      -- 'google' or 'github'
    provider_user_id VARCHAR(255) NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    token_expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_user_id)
);

CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(64) NOT NULL UNIQUE,  -- SHA-256 of token
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ
);

CREATE INDEX idx_oauth_accounts_user ON oauth_accounts(user_id);
CREATE INDEX idx_refresh_tokens_user ON refresh_tokens(user_id);
CREATE INDEX idx_refresh_tokens_hash ON refresh_tokens(token_hash);
```

### OAuth2 Client Configuration
```python
from authlib.integrations.starlette_client import OAuth

oauth = OAuth()

oauth.register(
    name='google',
    client_id=settings.GOOGLE_CLIENT_ID,
    client_secret=settings.GOOGLE_CLIENT_SECRET,
    server_metadata_url='https://accounts.google.com/.well-known/openid-configuration',
    client_kwargs={'scope': 'openid email profile'},
)

oauth.register(
    name='github',
    client_id=settings.GITHUB_CLIENT_ID,
    client_secret=settings.GITHUB_CLIENT_SECRET,
    authorize_url='https://github.com/login/oauth/authorize',
    access_token_url='https://github.com/login/oauth/access_token',
    api_base_url='https://api.github.com/',
    client_kwargs={'scope': 'user:email'},
)
```

### JWT Token Structure
```python
# Access Token Claims
{
    "sub": "user_uuid",           # Subject (user ID)
    "email": "user@example.com",
    "name": "User Name",
    "iat": 1704067200,            # Issued at
    "exp": 1704070800,            # Expires (1 hour)
    "iss": "auth.example.com",    # Issuer
    "aud": "api.example.com",     # Audience
    "jti": "unique_token_id"      # JWT ID for revocation
}

# Refresh Token Claims
{
    "sub": "user_uuid",
    "type": "refresh",
    "iat": 1704067200,
    "exp": 1704672000,            # 7 days
    "jti": "unique_refresh_id"
}
```

### Token Generation
```python
from datetime import datetime, timedelta
import jwt
from cryptography.hazmat.primitives import serialization

def create_access_token(user: User) -> str:
    now = datetime.utcnow()
    payload = {
        "sub": str(user.id),
        "email": user.email,
        "name": user.name,
        "iat": now,
        "exp": now + timedelta(hours=1),
        "iss": settings.JWT_ISSUER,
        "aud": settings.JWT_AUDIENCE,
        "jti": str(uuid.uuid4()),
    }
    return jwt.encode(payload, settings.JWT_PRIVATE_KEY, algorithm="RS256")

def create_refresh_token(user_id: UUID, db: AsyncSession) -> str:
    token_id = str(uuid.uuid4())
    expires_at = datetime.utcnow() + timedelta(days=7)

    # Store hash of token in DB
    token_hash = hashlib.sha256(token_id.encode()).hexdigest()
    refresh = RefreshToken(
        user_id=user_id,
        token_hash=token_hash,
        expires_at=expires_at,
    )
    db.add(refresh)
    await db.commit()

    payload = {
        "sub": str(user_id),
        "type": "refresh",
        "jti": token_id,
        "exp": expires_at,
    }
    return jwt.encode(payload, settings.JWT_PRIVATE_KEY, algorithm="RS256")
```

### OAuth Callback Handler
```python
@router.get("/google/callback")
async def google_callback(request: Request, db: AsyncSession = Depends(get_db)):
    token = await oauth.google.authorize_access_token(request)
    userinfo = token.get('userinfo')

    # Find or create user
    user = await get_or_create_user(
        db=db,
        email=userinfo['email'],
        name=userinfo.get('name'),
        avatar_url=userinfo.get('picture'),
        provider='google',
        provider_user_id=userinfo['sub'],
    )

    # Generate tokens
    access_token = create_access_token(user)
    refresh_token = await create_refresh_token(user.id, db)

    # Redirect with tokens (or return JSON for SPA)
    response = RedirectResponse(url=settings.FRONTEND_URL)
    response.set_cookie(
        "refresh_token",
        refresh_token,
        httponly=True,
        secure=True,
        samesite="lax",
        max_age=7 * 24 * 3600,
    )
    return response
```

### Token Validation Middleware
```python
async def get_current_user(
    authorization: str = Header(...),
    db: AsyncSession = Depends(get_db),
) -> User:
    if not authorization.startswith("Bearer "):
        raise HTTPException(401, "Invalid authorization header")

    token = authorization[7:]
    try:
        payload = jwt.decode(
            token,
            settings.JWT_PUBLIC_KEY,
            algorithms=["RS256"],
            audience=settings.JWT_AUDIENCE,
            issuer=settings.JWT_ISSUER,
        )
    except jwt.ExpiredSignatureError:
        raise HTTPException(401, "Token expired")
    except jwt.InvalidTokenError as e:
        raise HTTPException(401, f"Invalid token: {e}")

    user = await db.get(User, payload["sub"])
    if not user:
        raise HTTPException(401, "User not found")

    return user
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: OAuth2 Client Setup

**Summary:** Configure OAuth2 clients for Google and GitHub.

**Definition of Done:** Redirect URLs work and tokens are received.

### Ticket 2: User Management

**Summary:** Store and retrieve user profiles from database.

**Definition of Done:** Users are created on first login and retrieved on subsequent logins.

### Ticket 3: JWT Token Generation

**Summary:** Generate and validate JWT tokens.

**Definition of Done:** Tokens are correctly signed and validated.
