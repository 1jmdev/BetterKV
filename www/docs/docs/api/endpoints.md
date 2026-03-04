# API Endpoints

Full reference for all BetterKV REST API endpoints.

## String Operations

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/v1/keys/{key}` | Get value |
| `PUT` | `/v1/keys/{key}` | Set value |
| `DELETE` | `/v1/keys/{key}` | Delete key |
| `HEAD` | `/v1/keys/{key}` | Check existence |
| `GET` | `/v1/keys/{key}/ttl` | Get TTL |
| `PATCH` | `/v1/keys/{key}/expire` | Set TTL |
| `DELETE` | `/v1/keys/{key}/expire` | Remove TTL |

## Hash Operations

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/v1/hashes/{key}` | Get all fields (`HGETALL`) |
| `GET` | `/v1/hashes/{key}/{field}` | Get one field (`HGET`) |
| `PUT` | `/v1/hashes/{key}` | Set multiple fields (`HSET`) |
| `PUT` | `/v1/hashes/{key}/{field}` | Set one field |
| `DELETE` | `/v1/hashes/{key}/{field}` | Delete field (`HDEL`) |
| `GET` | `/v1/hashes/{key}/keys` | List fields (`HKEYS`) |

## List Operations

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/v1/lists/{key}` | Get all elements (`LRANGE 0 -1`) |
| `POST` | `/v1/lists/{key}/push` | Right-push (`RPUSH`) |
| `POST` | `/v1/lists/{key}/push?side=left` | Left-push (`LPUSH`) |
| `DELETE` | `/v1/lists/{key}/pop` | Right-pop (`RPOP`) |
| `DELETE` | `/v1/lists/{key}/pop?side=left` | Left-pop (`LPOP`) |
| `GET` | `/v1/lists/{key}/length` | List length (`LLEN`) |

## Set Operations

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/v1/sets/{key}` | All members (`SMEMBERS`) |
| `POST` | `/v1/sets/{key}` | Add members (`SADD`) |
| `DELETE` | `/v1/sets/{key}/{member}` | Remove member (`SREM`) |
| `GET` | `/v1/sets/{key}/{member}` | Check membership (`SISMEMBER`) |
| `GET` | `/v1/sets/{key}/count` | Cardinality (`SCARD`) |

## Sorted Set Operations

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/v1/zsets/{key}` | Range with scores (`ZRANGE WITHSCORES`) |
| `POST` | `/v1/zsets/{key}` | Add member (`ZADD`) |
| `DELETE` | `/v1/zsets/{key}/{member}` | Remove member (`ZREM`) |
| `GET` | `/v1/zsets/{key}/{member}/score` | Get score (`ZSCORE`) |
| `GET` | `/v1/zsets/{key}/{member}/rank` | Get rank (`ZRANK`) |
| `PATCH` | `/v1/zsets/{key}/{member}/score` | Increment score (`ZINCRBY`) |

## Batch Command

```http
POST /v1/cmd
```

Execute any command:

```json
{
  "cmd": ["ZADD", "lb", "1500", "alice"],
  "db": 0
}
```

Execute a pipeline (multiple commands):

```http
POST /v1/pipeline
```

```json
{
  "commands": [
    ["SET", "key1", "val1"],
    ["SET", "key2", "val2"],
    ["MGET", "key1", "key2"]
  ]
}
```

Response:

```json
{
  "ok": true,
  "results": [
    { "ok": true, "result": "OK" },
    { "ok": true, "result": "OK" },
    { "ok": true, "result": ["val1", "val2"] }
  ]
}
```

## Server Info

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/v1/ping` | Ping |
| `GET` | `/v1/info` | Server INFO |
| `GET` | `/v1/info/{section}` | Specific INFO section |
| `GET` | `/v1/dbsize` | Number of keys |
| `GET` | `/v1/health` | Health check (for load balancers) |

### Health Check

```bash
curl http://localhost:8080/v1/health
```

```json
{
  "status": "healthy",
  "version": "8.1.0",
  "uptime": 86400,
  "used_memory_human": "2.50M",
  "connected_clients": 42
}
```

Returns `200` when healthy, `503` when degraded.
