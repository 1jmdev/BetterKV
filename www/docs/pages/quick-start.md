# Quick Start

Get BetterKV running in under 60 seconds.

## Option 1 — Docker (Recommended)

```bash
docker run -d \
  --name betterkv \
  -p 6379:6379 \
  betterkv/betterkv:latest
```

Connect with any Redis client:

```bash
redis-cli -h localhost -p 6379 ping
# PONG
```

## Option 2 — Docker Compose

Create a `docker-compose.yml`:

```yaml title="docker-compose.yml"
version: "3.9"
services:
  betterkv:
    image: betterkv/betterkv:latest
    ports:
      - "6379:6379"
    volumes:
      - bkv-data:/data
    command: betterkv-server /etc/betterkv/betterkv.conf --save 60 1

volumes:
  bkv-data:
```

Then start it:

```bash
docker compose up -d
```

## Option 3 — Binary

Download from [GitHub Releases](https://github.com/1jmdev/BetterKV/releases) and run:

```bash
# macOS (arm64)
curl -Lo betterkv https://github.com/1jmdev/BetterKV/releases/latest/download/betterkv-macos-arm64
chmod +x betterkv && ./betterkv

# Linux (x86_64)
curl -Lo betterkv https://github.com/1jmdev/BetterKV/releases/latest/download/betterkv-linux-x86_64
chmod +x betterkv && ./betterkv
```

## Your First Commands

Once connected, try these:

```bash
# Store a value
SET greeting "Hello, BetterKV!"

# Retrieve it
GET greeting
# "Hello, BetterKV!"

# Set with expiry (seconds)
SET session:user1 "token-abc123" EX 3600

# Check TTL
TTL session:user1
# 3599

# Work with hashes
HSET user:1 name "Alice" email "alice@example.com" age "30"
HGETALL user:1

# Work with lists
RPUSH queue:tasks "task1" "task2" "task3"
LLEN queue:tasks
# 3

# Sorted set (leaderboard)
ZADD leaderboard 1500 "alice" 1200 "bob" 1800 "charlie"
ZREVRANGE leaderboard 0 2 WITHSCORES
```

## Using a Client Library

### Node.js (ioredis)

```bash
npm install ioredis
```

```js title="app.js"
import Redis from 'ioredis';

const client = new Redis({
  host: 'localhost',
  port: 6379,
});

await client.set('foo', 'bar');
const val = await client.get('foo');
console.log(val); // "bar"

await client.quit();
```

### Python (redis-py)

```bash
pip install redis
```

```python title="app.py"
import redis

r = redis.Redis(host='localhost', port=6379, decode_responses=True)

r.set('foo', 'bar')
print(r.get('foo'))  # bar
```

### Go (go-redis)

```bash
go get github.com/redis/go-redis/v9
```

```go title="main.go"
package main

import (
    "context"
    "fmt"
    "github.com/redis/go-redis/v9"
)

func main() {
    ctx := context.Background()
    rdb := redis.NewClient(&redis.Options{
        Addr: "localhost:6379",
    })

    err := rdb.Set(ctx, "foo", "bar", 0).Err()
    if err != nil { panic(err) }

    val, _ := rdb.Get(ctx, "foo").Result()
    fmt.Println(val) // bar
}
```

## Next Steps

- [Installation →](installation) — More install options including package managers
- [Configuration →](configuration) — Tune BetterKV for your workload
- [Data Types →](data-types) — Full guide to all supported data structures
