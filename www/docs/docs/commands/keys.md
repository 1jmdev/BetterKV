# Keys & Expiry Commands

Commands for managing key existence, types, expiry, and scanning.

## DEL

```
DEL key [key ...]
```

Deletes one or more keys. Returns the number of keys deleted.

**Complexity:** O(N) where N is the number of keys. Deleting a large collection (list, set, etc.) is O(M) where M is the number of elements.

```bash
SET foo "bar"
DEL foo           # 1
DEL foo missing   # 0 (already deleted)
DEL k1 k2 k3     # deletes all three
```

## UNLINK

```
UNLINK key [key ...]
```

Like `DEL` but performs the memory reclaim asynchronously in a background thread. Returns immediately.

**Complexity:** O(1) for the call; O(N) async for the reclaim

```bash
# Preferred over DEL for large keys in production
UNLINK large_list:items
```

## EXISTS

```
EXISTS key [key ...]
```

Returns the number of keys that exist. If you pass the same key multiple times, it's counted multiple times.

**Complexity:** O(N)

```bash
SET foo "bar"
EXISTS foo         # 1
EXISTS foo foo     # 2
EXISTS missing     # 0
EXISTS foo missing # 1
```

## TYPE

```
TYPE key
```

Returns the type of the value stored: `string`, `list`, `set`, `zset`, `hash`, `stream`, or `none`.

**Complexity:** O(1)

```bash
SET str_key "hello"        → TYPE str_key   # string
RPUSH list_key "a"         → TYPE list_key  # list
SADD set_key "x"           → TYPE set_key   # set
HSET hash_key f v          → TYPE hash_key  # hash
ZADD zset_key 1.0 "a"     → TYPE zset_key  # zset
TYPE missing_key           # none
```

## EXPIRE / PEXPIRE

```
EXPIRE key seconds [NX | XX | GT | LT]
PEXPIRE key milliseconds [NX | XX | GT | LT]
```

Set expiry on a key. Returns `1` if set, `0` if key doesn't exist.

**Complexity:** O(1) — BetterKV's EXPIRE is 3x faster than Redis

```bash
SET session "token"
EXPIRE session 3600        # expires in 1 hour
PEXPIRE session 60000      # expires in 60 seconds

# Options (added in 7.0):
EXPIRE key 100 NX          # only if NO expiry currently
EXPIRE key 100 XX          # only if expiry EXISTS
EXPIRE key 100 GT          # only if new TTL > current TTL
EXPIRE key 100 LT          # only if new TTL < current TTL
```

## TTL / PTTL

```
TTL key
PTTL key
```

Returns the remaining time-to-live in seconds (`TTL`) or milliseconds (`PTTL`).

- Returns `-2` if the key does not exist
- Returns `-1` if the key exists but has no expiry

**Complexity:** O(1)

```bash
SET foo "bar" EX 120
TTL foo     # 119 (approximately)
PTTL foo    # 118943 (ms)

SET permanent "value"
TTL permanent   # -1

TTL missing     # -2
```

## EXPIREAT / PEXPIREAT

```
EXPIREAT key unix-timestamp
PEXPIREAT key unix-timestamp-ms
```

Set expiry as an absolute Unix timestamp.

**Complexity:** O(1)

```bash
# Expire at midnight 2027-01-01
EXPIREAT cache:data 1767225600

# Expire at specific ms timestamp
PEXPIREAT cache:data 1767225600000
```

## PERSIST

```
PERSIST key
```

Remove the expiry from a key, making it permanent. Returns `1` if TTL was removed, `0` if key doesn't exist or has no TTL.

**Complexity:** O(1)

```bash
SET session "token" EX 3600
PERSIST session
TTL session     # -1 (permanent)
```

## RENAME / RENAMENX

```
RENAME key newkey
RENAMENX key newkey
```

Rename `key` to `newkey`. `RENAMENX` only renames if `newkey` doesn't exist.

**Complexity:** O(1)

```bash
SET old_name "Alice"
RENAME old_name name
GET name         # "Alice"
GET old_name     # (nil)
```

## COPY

```
COPY source destination [DB db] [REPLACE]
```

Copies the value of `source` to `destination`. Does not delete `source`.

**Complexity:** O(N) for collection types

```bash
SET original "hello"
COPY original backup         # 1 (success)
COPY original backup         # 0 (destination exists)
COPY original backup REPLACE # 1 (overwrites)
COPY original other DB 1     # copy to database 1
```

## SCAN

```
SCAN cursor [MATCH pattern] [COUNT count] [TYPE type]
```

Incrementally iterate over all keys. Use in production instead of `KEYS *` — `SCAN` is non-blocking.

**Complexity:** O(1) per call, O(N) total

```bash
# Iterate all keys
SCAN 0
# 1) "48"          -- next cursor
# 2) 1) "key1"    -- returned keys
#    2) "key2"

# Continue until cursor returns 0
SCAN 48
# 1) "0"           -- done
# 2) 1) "key3"

# With filters
SCAN 0 MATCH "user:*" COUNT 100
SCAN 0 TYPE hash
```

## KEYS (avoid in production)

```
KEYS pattern
```

Returns all keys matching a glob pattern. **Blocks the server** while scanning. Use `SCAN` in production.

**Complexity:** O(N)

```bash
KEYS *           # all keys (dangerous in production)
KEYS user:*      # all keys starting with "user:"
KEYS *:session   # all keys ending with ":session"
KEYS user:?:*    # user:1:token, user:2:profile, etc.
```

## OBJECT

```
OBJECT ENCODING key
OBJECT IDLETIME key
OBJECT FREQ key
OBJECT HELP
```

Inspect internal object encoding and usage stats.

```bash
SET num 12345
OBJECT ENCODING num     # "int"

SET str "hello world"
OBJECT ENCODING str     # "embstr"

RPUSH mylist a b c
OBJECT ENCODING mylist  # "listpack" or "quicklist"
```
