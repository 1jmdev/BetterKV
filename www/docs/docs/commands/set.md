# Set Commands

Sets are unordered collections of unique strings. All membership tests are O(1).

## SADD / SREM

```
SADD key member [member ...]
SREM key member [member ...]
```

Add or remove members. Returns the number of members added/removed.

**Complexity:** O(1) per member

```bash
SADD tags "redis" "database" "cache"    # 3
SADD tags "redis"                       # 0 (already exists)
SREM tags "cache"                       # 1
```

## SISMEMBER / SMISMEMBER

```
SISMEMBER key member
SMISMEMBER key member [member ...]
```

Check membership for one or many members.

**Complexity:** O(1) per member

```bash
SADD fruits apple banana cherry
SISMEMBER fruits apple    # 1
SISMEMBER fruits grape    # 0

SMISMEMBER fruits apple grape banana
# 1) 1
# 2) 0
# 3) 1
```

## SMEMBERS

```
SMEMBERS key
```

Returns all members. **Avoid on large sets** — use `SSCAN` instead.

**Complexity:** O(N)

```bash
SMEMBERS tags
# 1) "redis"
# 2) "database"
```

## SCARD

```
SCARD key
```

Returns the number of members (cardinality).

**Complexity:** O(1)

```bash
SCARD tags     # 2
SCARD missing  # 0
```

## Set Operations

### SUNION / SUNIONSTORE

```
SUNION key [key ...]
SUNIONSTORE destination key [key ...]
```

Union of all sets. `SUNIONSTORE` saves the result.

**Complexity:** O(N) where N is total members across all sets

```bash
SADD team:a alice charlie
SADD team:b bob charlie

SUNION team:a team:b           # [alice, charlie, bob]
SUNIONSTORE team:all team:a team:b
SMEMBERS team:all              # [alice, charlie, bob]
```

### SINTER / SINTERSTORE / SINTERCARD

```
SINTER key [key ...]
SINTERSTORE destination key [key ...]
SINTERCARD numkeys key [key ...] [LIMIT limit]
```

Intersection — members present in ALL sets.

**Complexity:** O(N*M) where N = smallest set, M = number of sets

```bash
SINTER team:a team:b           # [charlie]
SINTERCARD 2 team:a team:b     # 1 (count only)
SINTERCARD 2 team:a team:b LIMIT 10
```

### SDIFF / SDIFFSTORE

```
SDIFF key [key ...]
SDIFFSTORE destination key [key ...]
```

Difference — members in first set but not in others.

**Complexity:** O(N) where N is total members

```bash
SDIFF team:a team:b   # [alice] (in a, not in b)
SDIFF team:b team:a   # [bob]   (in b, not in a)
```

## SPOP / SRANDMEMBER

```
SPOP key [count]
SRANDMEMBER key [count]
```

`SPOP` — remove and return random member(s).  
`SRANDMEMBER` — return random member(s) without removing.

**Complexity:** O(1) for single, O(N) for count

```bash
SADD lottery a b c d e

SPOP lottery          # "c" (removed from set)
SPOP lottery 2        # ["a", "b"] (both removed)

SRANDMEMBER lottery   # "d" (not removed)
SRANDMEMBER lottery 2 # 2 unique random members
SRANDMEMBER lottery -5 # 5 random (may repeat)
```

## SMOVE

```
SMOVE source destination member
```

Atomically move a member from one set to another.

**Complexity:** O(1)

```bash
SADD set:a "item"
SMOVE set:a set:b "item"
# item is now in set:b, removed from set:a
```

## SSCAN

```
SSCAN key cursor [MATCH pattern] [COUNT count]
```

Incrementally iterate over set members.

**Complexity:** O(1) per call, O(N) total

```bash
SSCAN myset 0 MATCH "user:*" COUNT 100
```
