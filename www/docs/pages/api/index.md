# REST API Overview

BetterKV includes an optional HTTP REST API layer, perfect for environments where a Redis client library isn't available or for quick integrations.

:::info
The REST API is a BetterKV extension. Enable it in your config — it is disabled by default.
:::

## Enabling the REST API

```ini title="betterkv.conf"
# Enable REST API
rest-api-enabled yes
rest-api-port 8080
rest-api-bind 127.0.0.1

# Authentication
rest-api-token your-secret-api-token
```

Or via Docker:

```bash
docker run -d \
  -p 6379:6379 \
  -p 8080:8080 \
  -e BKV_REST_API=1 \
  -e BKV_REST_TOKEN=my-token \
  betterkv/betterkv:latest
```

## Base URL & Auth

```
Base URL: http://localhost:8080/v1
```

All requests require the `Authorization` header:

```http
Authorization: Bearer your-secret-api-token
```

## Response Format

All responses are JSON:

```json
{
  "ok": true,
  "result": "PONG",
  "type": "string"
}
```

Error responses:

```json
{
  "ok": false,
  "error": "ERR key does not exist",
  "code": 404
}
```

## Core Endpoints

### Ping

```http
GET /v1/ping
```

```bash
curl http://localhost:8080/v1/ping \
  -H "Authorization: Bearer my-token"

# Response:
{ "ok": true, "result": "PONG" }
```

### GET

```http
GET /v1/keys/{key}
```

```bash
curl http://localhost:8080/v1/keys/user:1 \
  -H "Authorization: Bearer my-token"

# Response:
{ "ok": true, "result": "Alice", "type": "string" }
```

### SET

```http
PUT /v1/keys/{key}
Content-Type: application/json
```

```bash
curl -X PUT http://localhost:8080/v1/keys/user:1 \
  -H "Authorization: Bearer my-token" \
  -H "Content-Type: application/json" \
  -d '{"value": "Alice", "ex": 3600}'

# Response:
{ "ok": true, "result": "OK" }
```

Request body:

| Field | Type | Description |
|-------|------|-------------|
| `value` | string | The value to set |
| `ex` | number | Expiry in seconds |
| `px` | number | Expiry in milliseconds |
| `nx` | boolean | Only set if not exists |
| `xx` | boolean | Only set if exists |

### DELETE

```http
DELETE /v1/keys/{key}
```

```bash
curl -X DELETE http://localhost:8080/v1/keys/user:1 \
  -H "Authorization: Bearer my-token"

# Response:
{ "ok": true, "result": 1 }
```

### EXISTS

```http
HEAD /v1/keys/{key}
```

Returns `200` if key exists, `404` if not.

### TTL

```http
GET /v1/keys/{key}/ttl
```

```bash
curl http://localhost:8080/v1/keys/session:abc/ttl \
  -H "Authorization: Bearer my-token"

# Response:
{ "ok": true, "result": 3542, "unit": "seconds" }
```

### EXPIRE

```http
PATCH /v1/keys/{key}/expire
Content-Type: application/json
```

```bash
curl -X PATCH http://localhost:8080/v1/keys/session:abc/expire \
  -H "Authorization: Bearer my-token" \
  -H "Content-Type: application/json" \
  -d '{"seconds": 7200}'
```

## Command Execution Endpoint

For full flexibility, execute any command via the generic endpoint:

```http
POST /v1/cmd
Content-Type: application/json
```

```bash
curl -X POST http://localhost:8080/v1/cmd \
  -H "Authorization: Bearer my-token" \
  -H "Content-Type: application/json" \
  -d '{"cmd": ["ZADD", "leaderboard", "1500", "alice"]}'

# Response:
{ "ok": true, "result": 1, "type": "integer" }
```

## Rate Limits

| Plan | Requests/min | Max payload |
|------|-------------|-------------|
| Free | 1,000 | 1 MB |
| Pro | 10,000 | 10 MB |
| Enterprise | Unlimited | 100 MB |

See [Authentication →](/api/auth) for plan details.
