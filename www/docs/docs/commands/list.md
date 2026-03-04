# List Commands

Lists are ordered sequences of strings. They support efficient push/pop at both ends and are often used as queues or stacks.

## RPUSH / LPUSH

```
RPUSH key element [element ...]
LPUSH key element [element ...]
```

Push elements to the right (tail) or left (head). Returns the list length after the push.

**Complexity:** O(1) per element

```bash
RPUSH queue "task1" "task2" "task3"
# queue: [task1, task2, task3]

LPUSH queue "urgent"
# queue: [urgent, task1, task2, task3]
```

`RPUSHX` / `LPUSHX` — only push if the key exists:

```bash
RPUSHX queue "item"   # only if queue exists
```

## RPOP / LPOP

```
RPOP key [count]
LPOP key [count]
```

Remove and return elements from tail or head. Optionally return multiple elements.

**Complexity:** O(N) where N is count

```bash
RPOP queue          # "task3"
LPOP queue          # "urgent"
LPOP queue 2        # ["task1", "task2"]
```

## LRANGE

```
LRANGE key start stop
```

Return a range of elements. Indices are 0-based; `-1` means last element. **Does not modify the list.**

**Complexity:** O(N) where N is the number of elements returned

```bash
RPUSH list a b c d e
LRANGE list 0 -1    # [a, b, c, d, e] — all
LRANGE list 0 2     # [a, b, c]
LRANGE list -3 -1   # [c, d, e]
LRANGE list 1 3     # [b, c, d]
```

## LLEN

```
LLEN key
```

Returns the length of the list. Returns `0` if key does not exist.

**Complexity:** O(1)

```bash
LLEN queue    # 5
LLEN missing  # 0
```

## LINDEX / LSET

```
LINDEX key index
LSET key index element
```

Get or set element at an index.

**Complexity:** O(N) — traverses to the index

```bash
RPUSH list a b c d
LINDEX list 0     # "a"
LINDEX list -1    # "d"
LINDEX list 10    # (nil)

LSET list 1 "B"
LRANGE list 0 -1  # [a, B, c, d]
```

## LINSERT

```
LINSERT key BEFORE | AFTER pivot element
```

Insert `element` before or after the first occurrence of `pivot`.

**Complexity:** O(N)

```bash
RPUSH list a b c
LINSERT list BEFORE b "X"
LRANGE list 0 -1   # [a, X, b, c]
```

## LREM

```
LREM key count element
```

Remove `count` occurrences of `element`:
- `count > 0`: from head to tail
- `count < 0`: from tail to head
- `count = 0`: all occurrences

**Complexity:** O(N)

```bash
RPUSH list a b a c a
LREM list 2 a        # removes first 2 "a"s → [b, c, a]
LREM list -1 a       # removes last "a" → [b, c]
LREM list 0 b        # removes all "b"s
```

## LTRIM

```
LTRIM key start stop
```

Trim the list to only keep elements in the given range. Useful for keeping lists bounded.

**Complexity:** O(N) where N is elements removed

```bash
RPUSH list a b c d e
LTRIM list 0 2        # keep first 3 only
LRANGE list 0 -1      # [a, b, c]

# Keep last 100 items (common pattern)
RPUSH log "entry"
LTRIM log -100 -1
```

## LMOVE

```
LMOVE source destination LEFT | RIGHT LEFT | RIGHT
```

Atomically pop from one list and push to another.

**Complexity:** O(1)

```bash
RPUSH source a b c
LMOVE source dest LEFT RIGHT
# source: [b, c], dest: [a]

# Rotate: move last element to front
LMOVE mylist mylist RIGHT LEFT
```

## BLPOP / BRPOP

```
BLPOP key [key ...] timeout
BRPOP key [key ...] timeout
```

Blocking pop — waits up to `timeout` seconds for an element to be available. Returns `nil` on timeout.

**Complexity:** O(1)

```bash
# Block up to 30 seconds
BLPOP queue:jobs 30

# Block on multiple queues (priority order)
BLPOP queue:urgent queue:normal queue:low 10

# Block indefinitely (0 = no timeout)
BLPOP queue 0
```

**Pattern: Worker Queue**

```js
// Worker process
while (true) {
  const [queue, job] = await client.blpop('queue:jobs', 0);
  await processJob(JSON.parse(job));
}
```

## BLMOVE

```
BLMOVE source destination LEFT | RIGHT LEFT | RIGHT timeout
```

Blocking version of `LMOVE`. Reliable queue pattern.

```bash
# Reliable queue: move from pending to processing
BLMOVE jobs:pending jobs:processing LEFT LEFT 30
```
