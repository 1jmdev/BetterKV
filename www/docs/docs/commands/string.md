# String Commands

String commands operate on string values (up to 512 MB).

## GET

```
GET key
```

Returns the value of `key`. Returns `nil` if the key does not exist.

**Complexity:** O(1)

```bash
SET greeting "Hello"
GET greeting    # "Hello"
GET missing     # (nil)
```

## SET

```
SET key value [NX | XX] [GET] [EX seconds | PX milliseconds | EXAT timestamp | PXAT ms-timestamp | KEEPTTL]
```

Sets `key` to `value`. Replaces any existing value, including a different type.

**Complexity:** O(1)

```bash
SET name "Alice"
SET counter 0
SET session:x "token" EX 3600   # expires in 1 hour
SET lock "1" NX                  # only if not exists
SET flag "on" XX                 # only if exists

# GET option: return old value
SET name "Bob" GET               # returns "Alice"
```

## MGET / MSET

```
MGET key [key ...]
MSET key value [key value ...]
```

Get or set multiple keys in a single round trip.

**Complexity:** O(N) where N is the number of keys

```bash
MSET user:1:name "Alice" user:2:name "Bob" user:3:name "Charlie"
MGET user:1:name user:2:name user:3:name
# 1) "Alice"
# 2) "Bob"
# 3) "Charlie"
```

`MSETNX` — set multiple keys only if none of them exist:

```bash
MSETNX key1 "val1" key2 "val2"
```

## APPEND

```
APPEND key value
```

Appends `value` to the string stored at `key`. Returns the length of the resulting string.

**Complexity:** O(1) amortized

```bash
SET log ""
APPEND log "2026-03-04 "    # 11
APPEND log "INFO server "   # 23
APPEND log "started\n"      # 31
```

## STRLEN

```
STRLEN key
```

Returns the length of the string stored at `key`.

**Complexity:** O(1)

```bash
SET greeting "Hello, World!"
STRLEN greeting    # 13
STRLEN missing     # 0
```

## GETRANGE / SETRANGE

```
GETRANGE key start end
SETRANGE key offset value
```

Get or overwrite a substring. Ranges are inclusive, 0-indexed. Negative indices count from end.

**Complexity:** O(N) where N is the length of the returned/modified string

```bash
SET greeting "Hello, World!"
GETRANGE greeting 0 4     # "Hello"
GETRANGE greeting 7 11    # "World"
GETRANGE greeting -6 -1   # "World!"

SETRANGE greeting 7 "BetterKV"
GET greeting              # "Hello, BetterKV"
```

## GETDEL / GETEX

```
GETDEL key
GETEX key [EX seconds | PX milliseconds | EXAT timestamp | PERSIST]
```

`GETDEL` — get and atomically delete.  
`GETEX` — get and optionally update the expiry.

**Complexity:** O(1)

```bash
SET token "abc123"

GETDEL token          # "abc123", key deleted
GETEX session "abc" EX 3600   # refresh TTL to 1 hour
GETEX session "abc" PERSIST   # remove TTL (persist forever)
```

## SETNX / SETEX / PSETEX

Legacy commands (prefer `SET` with options):

```bash
SETNX key value       # SET key value NX
SETEX key seconds value  # SET key value EX seconds
PSETEX key ms value   # SET key value PX ms
```

## SUBSTR (deprecated)

Use `GETRANGE` instead. `SUBSTR` is kept for compatibility.
