# Commands Reference

Complete reference for all BetterKV commands. BetterKV is fully compatible with the Redis command set.

## Command Notation

```
COMMAND key [optional] <required> [one|two]
```

- `key` — required argument
- `[optional]` — optional argument
- `<required>` — required choice
- `[one|two]` — optional, choose one

## Complexity Notation

Time complexity uses Big O notation over the number of elements:

| Notation | Meaning |
|----------|---------|
| O(1) | Constant time — instant regardless of size |
| O(N) | Linear — scales with number of elements |
| O(log N) | Logarithmic — very efficient at scale |
| O(N log N) | Log-linear |

## Categories

| Category | Commands |
|----------|---------|
| [Strings](/commands/string) | GET, SET, MGET, MSET, INCR, APPEND, STRLEN, ... |
| [Numeric](/commands/numeric) | INCR, INCRBY, DECR, DECRBY, INCRBYFLOAT, ... |
| [Lists](/commands/list) | RPUSH, LPOP, LRANGE, LLEN, BLPOP, ... |
| [Hashes](/commands/hash) | HSET, HGET, HGETALL, HDEL, HINCRBY, ... |
| [Sets](/commands/set) | SADD, SREM, SMEMBERS, SINTER, SUNION, ... |
| [Sorted Sets](/commands/sorted-set) | ZADD, ZRANGE, ZRANK, ZSCORE, ZCOUNT, ... |
| [Keys & Expiry](/commands/keys) | DEL, EXISTS, EXPIRE, TTL, RENAME, TYPE, ... |
| [Server](/commands/server) | INFO, DBSIZE, CONFIG, FLUSHDB, DEBUG, ... |

## Global Options

These options apply to many write commands:

### EX / PX / EXAT / PXAT — Expiry

```bash
SET key value EX 60        # expire in 60 seconds
SET key value PX 60000     # expire in 60 milliseconds
SET key value EXAT 1735689600  # expire at unix timestamp (seconds)
SET key value PXAT 1735689600000  # expire at unix timestamp (ms)
```

### NX / XX — Conditional Write

```bash
SET key value NX   # only set if key does NOT exist
SET key value XX   # only set if key DOES exist
```

### KEEPTTL — Preserve TTL

```bash
SET key "new_value" KEEPTTL  # update value but keep existing TTL
```
