# Authentication

The BetterKV REST API uses Bearer token authentication.

## Obtaining a Token

Tokens are configured in your `betterkv.conf`:

```ini title="betterkv.conf"
# Single token
rest-api-token your-api-token-here

# Multiple tokens (comma-separated)
rest-api-tokens token1,token2,token3

# Token with permissions
rest-api-token-read readonly-token
rest-api-token-write readwrite-token
```

## Using the Token

Include the token in every request:

```bash
# Authorization header (recommended)
curl https://your-server/v1/ping \
  -H "Authorization: Bearer your-api-token"

# Query parameter (less secure, avoid in production)
curl "https://your-server/v1/ping?token=your-api-token"
```

## Token Permissions

| Token Type | GET | SET | DELETE | CONFIG | ADMIN |
|-----------|-----|-----|--------|--------|-------|
| Read-only | ✓ | ✗ | ✗ | ✗ | ✗ |
| Read-write | ✓ | ✓ | ✓ | ✗ | ✗ |
| Admin | ✓ | ✓ | ✓ | ✓ | ✓ |

## Error Responses

| HTTP Status | Meaning |
|------------|---------|
| `401 Unauthorized` | Token missing |
| `403 Forbidden` | Token invalid or insufficient permissions |
| `429 Too Many Requests` | Rate limit exceeded |

```json
{
  "ok": false,
  "error": "Invalid or missing API token",
  "code": 401
}
```

## Token Rotation

Rotate tokens without downtime using multiple tokens:

```ini title="betterkv.conf"
# Both tokens are valid during transition
rest-api-tokens old-token,new-token
```

Once all clients are updated to the new token, remove the old one and reload config:

```bash
betterkv-cli CONFIG REWRITE
betterkv-cli CONFIG REWRITE  # applies without restart
```
