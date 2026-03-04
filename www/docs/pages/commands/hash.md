# Hash Commands

Hashes store field-value pairs under a single key. Efficient for representing objects.

## HSET / HMSET

```
HSET key field value [field value ...]
```

Set one or more fields. Returns the number of new fields added.

**Complexity:** O(1) per field

```bash
HSET user:1 name "Alice" email "alice@example.com" age "30"
# (integer) 3

# Update a single field
HSET user:1 age "31"
# (integer) 0  (field existed, updated in place)
```

`HMSET` is an alias (deprecated in Redis 4.0, but still supported).

## HGET / HMGET

```
HGET key field
HMGET key field [field ...]
```

Get one or multiple field values.

**Complexity:** O(1) per field

```bash
HGET user:1 name      # "Alice"
HGET user:1 missing   # (nil)

HMGET user:1 name email age
# 1) "Alice"
# 2) "alice@example.com"
# 3) "31"
```

## HGETALL

```
HGETALL key
```

Returns all field-value pairs as a flat list.

**Complexity:** O(N) where N is the number of fields

```bash
HGETALL user:1
# 1) "name"
# 2) "Alice"
# 3) "email"
# 4) "alice@example.com"
# 5) "age"
# 6) "31"
```

## HKEYS / HVALS / HLEN

```
HKEYS key
HVALS key
HLEN key
```

Get all field names, all values, or the number of fields.

**Complexity:** O(N)

```bash
HKEYS user:1   # ["name", "email", "age"]
HVALS user:1   # ["Alice", "alice@example.com", "31"]
HLEN user:1    # 3
```

## HDEL

```
HDEL key field [field ...]
```

Delete one or more fields. Returns number of fields deleted.

**Complexity:** O(1) per field

```bash
HDEL user:1 age          # 1
HDEL user:1 x y          # 0 (neither existed)
```

## HEXISTS

```
HEXISTS key field
```

Returns `1` if the field exists, `0` otherwise.

**Complexity:** O(1)

```bash
HEXISTS user:1 name     # 1
HEXISTS user:1 phone    # 0
```

## HINCRBY / HINCRBYFLOAT

```
HINCRBY key field increment
HINCRBYFLOAT key field increment
```

Increment a numeric field atomically.

**Complexity:** O(1)

```bash
HSET stats page:home views 0 likes 0
HINCRBY stats page:home views 1         # 1
HINCRBY stats page:home views 5         # 6
HINCRBYFLOAT stats page:home rating 0.5 # "0.5"
```

## HSCAN

```
HSCAN key cursor [MATCH pattern] [COUNT count]
```

Incrementally iterate over hash fields. Use instead of `HGETALL` for very large hashes.

**Complexity:** O(1) per call, O(N) total

```bash
HSCAN user:1 0
# 1) "0"               -- cursor (done)
# 2) 1) "name"
#    2) "Alice"
#    3) "email"
#    4) "alice@example.com"

# With pattern filter
HSCAN big_hash 0 MATCH "user:*" COUNT 50
```

## HRANDFIELD

```
HRANDFIELD key [count [WITHVALUES]]
```

Return random field(s) from the hash.

**Complexity:** O(N) where N is the count

```bash
HRANDFIELD user:1           # "email" (random field)
HRANDFIELD user:1 2         # 2 unique random fields
HRANDFIELD user:1 -5        # 5 random fields (may repeat)
HRANDFIELD user:1 2 WITHVALUES  # fields + values
```
