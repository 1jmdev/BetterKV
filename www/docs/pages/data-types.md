# Data Types

BetterKV supports all standard Redis data types plus some extensions. Each type is optimized for specific use cases.

## Strings

The most basic type — a sequence of bytes up to 512 MB.

```bash
SET user:name "Alice"
GET user:name          # "Alice"

# Integer operations
SET counter 0
INCR counter           # 1
INCRBY counter 10      # 11
DECR counter           # 10

# With expiry
SET token "abc123" EX 3600        # expire in 1 hour
SET token "abc123" PX 60000       # expire in 60 seconds
SET token "abc123" EXAT 1735689600 # expire at unix timestamp

# Conditional sets
SET lock "acquired" NX            # only if NOT exists
SET config "v2" XX                # only if EXISTS

# Atomic get-and-set
GETSET old_key "new_value"
GETDEL key                        # get and delete atomically
```

**Use cases:** session tokens, counters, rate limiting, feature flags, caching.

## Lists

Ordered sequences of strings, implemented as a doubly linked list.

```bash
# Push / pop from both ends
RPUSH queue "task1" "task2" "task3"
LPUSH queue "urgent"

RPOP queue             # "task3"
LPOP queue             # "urgent"

# Range (0-indexed, -1 = last)
LRANGE queue 0 -1      # all elements
LRANGE queue 0 1       # first two

# Length
LLEN queue

# Blocking pop (great for worker queues)
BLPOP queue 30         # block up to 30 seconds
BRPOP queue 0          # block indefinitely
```

**Use cases:** message queues, activity feeds, job queues, chat history.

## Hashes

Field-value maps stored under a single key.

```bash
# Set multiple fields
HSET user:1 name "Alice" email "alice@example.com" role "admin"

# Get one or all
HGET user:1 name
HGETALL user:1

# Field operations
HDEL user:1 role
HEXISTS user:1 email   # 1 (true)
HLEN user:1
HKEYS user:1
HVALS user:1

# Numeric field operations
HSET stats hits 0
HINCRBY stats hits 1
```

**Use cases:** user profiles, product records, session state, configuration objects.

## Sets

Unordered collections of unique strings.

```bash
# Add members
SADD online:users "alice" "bob" "charlie"

# Check membership (O(1))
SISMEMBER online:users "alice"   # 1

# Remove
SREM online:users "bob"

# Set operations
SADD team:a "alice" "charlie"
SADD team:b "bob" "charlie"

SUNION team:a team:b         # all members
SINTER team:a team:b         # shared members
SDIFF  team:a team:b         # in a but not b

# Random sampling
SRANDMEMBER online:users 3   # 3 random (no removal)
SPOP online:users            # remove and return random
```

**Use cases:** tags, followers/following, unique visitors, set operations, recommendations.

## Sorted Sets

Like Sets, but each member has a floating-point score used for ordering.

```bash
# Add with score
ZADD leaderboard 1500 "alice"
ZADD leaderboard 1200 "bob" 1800 "charlie"

# Range by rank (lowest score first)
ZRANGE leaderboard 0 -1 WITHSCORES

# Range by rank (highest score first)
ZREVRANGE leaderboard 0 2 WITHSCORES

# Range by score
ZRANGEBYSCORE leaderboard 1400 1900

# Rank of a member (0 = lowest)
ZRANK leaderboard "alice"
ZREVRANK leaderboard "alice"   # rank from top

# Update score
ZINCRBY leaderboard 50 "alice"

# Count in score range
ZCOUNT leaderboard 1400 2000
```

**Use cases:** leaderboards, priority queues, rate limiting with time windows, autocomplete.

## Bitmaps

Bit-level operations on string values.

```bash
# Set bit at offset
SETBIT user:1:logins 0 1   # logged in on day 0
SETBIT user:1:logins 5 1   # logged in on day 5

# Get bit
GETBIT user:1:logins 5     # 1

# Count set bits
BITCOUNT user:1:logins     # 2

# Bitwise operations across keys
BITOP AND result key1 key2
BITOP OR  result key1 key2
```

**Use cases:** user activity tracking, feature flags, bloom filter approximation.

## HyperLogLog

Probabilistic data structure for cardinality estimation. Uses ~12 KB regardless of the number of unique elements, with a 0.81% standard error.

```bash
PFADD unique:visitors "user1" "user2" "user3"
PFADD unique:visitors "user1"   # duplicate, no change

PFCOUNT unique:visitors   # ≈ 3

# Merge multiple HyperLogLogs
PFMERGE total:visitors day1:visitors day2:visitors
```

**Use cases:** unique visitor counting, distinct event tracking at scale.

## Streams

Append-only log structure for event streaming.

```bash
# Append events (auto-generates IDs)
XADD events * action "login" user "alice"
XADD events * action "purchase" item "widget" amount "29.99"

# Read range
XRANGE events - +           # all events
XRANGE events - + COUNT 10  # first 10

# Read latest N
XREVRANGE events + - COUNT 5

# Consumer groups (for distributed processing)
XGROUP CREATE events workers $ MKSTREAM
XREADGROUP GROUP workers consumer1 COUNT 10 STREAMS events >
XACK events workers "1234567890-0"
```

**Use cases:** event sourcing, activity logs, distributed messaging, audit trails.
