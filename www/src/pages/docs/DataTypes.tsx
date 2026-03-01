import { PageHeader } from "@/components/PageHeader";
import { CodeBlock } from "@/components/CodeBlock";
import { DocsPager } from "@/components/DocsPager";

export function DocsDataTypes() {
  return (
    <div>
      <PageHeader
        title="Data Types"
        description="JustKV supports the core Redis data types. Each type has its own set of commands for manipulation."
      />

      <div className="prose-docs mt-8 space-y-10">
        <section>
          <h2>Strings</h2>
          <p>
            The most basic data type. A string value can be at most 512 MB.
            Strings can hold any binary data including serialized objects, images,
            or simple text and numbers.
          </p>
          <CodeBlock
            title="String operations"
            code={`# Basic set/get
SET key "value"
GET key

# Set with expiration (seconds)
SET session:token "abc123" EX 3600

# Set with expiration (milliseconds)
SET session:token "abc123" PX 60000

# Set only if key does not exist
SET lock:resource "owner1" NX

# Set only if key already exists
SET lock:resource "owner2" XX

# Atomic increment/decrement
SET counter 100
INCR counter          # 101
INCRBY counter 10     # 111
DECR counter          # 110
DECRBY counter 5      # 105
INCRBYFLOAT counter 1.5  # 106.5

# Append to a string
APPEND key " more data"

# Get string length
STRLEN key

# Set/get multiple keys
MSET key1 "val1" key2 "val2" key3 "val3"
MGET key1 key2 key3`}
          />
        </section>

        <section>
          <h2>Lists</h2>
          <p>
            Ordered collections of strings, implemented as linked lists. They
            support push/pop operations from both ends, making them suitable for
            queues, stacks, and timeline-style data.
          </p>
          <CodeBlock
            title="List operations"
            code={`# Push elements to the head (left)
LPUSH queue "task3" "task2" "task1"

# Push elements to the tail (right)
RPUSH queue "task4" "task5"

# Pop from head and tail
LPOP queue    # "task1"
RPOP queue    # "task5"

# Get elements by range (0-indexed)
LRANGE queue 0 -1    # all elements
LRANGE queue 0 2     # first 3 elements

# Get list length
LLEN queue

# Get element by index
LINDEX queue 0

# Set element at index
LSET queue 0 "updated-task"

# Trim list to a range
LTRIM queue 0 99     # keep first 100 elements

# Blocking pop (useful for work queues)
BLPOP queue 30       # wait up to 30 seconds
BRPOP queue 30`}
          />
        </section>

        <section>
          <h2>Sets</h2>
          <p>
            Unordered collections of unique strings. Sets support membership
            testing, intersection, union, and difference operations.
          </p>
          <CodeBlock
            title="Set operations"
            code={`# Add members
SADD tags "rust" "database" "kv-store" "open-source"

# Check membership
SISMEMBER tags "rust"        # 1 (true)
SISMEMBER tags "python"      # 0 (false)

# Get all members
SMEMBERS tags

# Get number of members
SCARD tags

# Remove members
SREM tags "open-source"

# Get random member(s)
SRANDMEMBER tags
SRANDMEMBER tags 2

# Pop random member
SPOP tags

# Set operations
SADD set1 "a" "b" "c"
SADD set2 "b" "c" "d"
SINTER set1 set2       # "b" "c"
SUNION set1 set2       # "a" "b" "c" "d"
SDIFF set1 set2        # "a"`}
          />
        </section>

        <section>
          <h2>Hashes</h2>
          <p>
            Maps between string fields and string values. Ideal for representing
            objects with multiple attributes.
          </p>
          <CodeBlock
            title="Hash operations"
            code={`# Set fields
HSET user:1 name "Alice" email "alice@example.com" role "admin"

# Get a single field
HGET user:1 name           # "Alice"

# Get multiple fields
HMGET user:1 name email    # "Alice" "alice@example.com"

# Get all fields and values
HGETALL user:1

# Check if field exists
HEXISTS user:1 email       # 1

# Get all field names
HKEYS user:1

# Get all values
HVALS user:1

# Get number of fields
HLEN user:1

# Delete fields
HDEL user:1 role

# Increment a numeric field
HSET user:1 login_count 0
HINCRBY user:1 login_count 1

# Set field only if it doesn't exist
HSETNX user:1 created_at "2026-02-28"`}
          />
        </section>

        <section>
          <h2>Sorted Sets</h2>
          <p>
            Similar to sets, but each member has an associated score (a floating
            point number). Members are ordered by score, making sorted sets ideal
            for leaderboards, rate limiters, and priority queues.
          </p>
          <CodeBlock
            title="Sorted set operations"
            code={`# Add members with scores
ZADD leaderboard 1500 "alice" 1200 "bob" 1800 "charlie" 900 "dave"

# Get members by rank (ascending score)
ZRANGE leaderboard 0 -1               # all, lowest to highest
ZRANGE leaderboard 0 2                # top 3 lowest

# Get members by rank (descending score)
ZREVRANGE leaderboard 0 2             # top 3 highest

# Get members with scores
ZRANGE leaderboard 0 -1 WITHSCORES

# Get rank of a member (0-indexed)
ZRANK leaderboard "alice"             # rank by ascending score
ZREVRANK leaderboard "alice"          # rank by descending score

# Get score of a member
ZSCORE leaderboard "alice"            # 1500

# Increment score
ZINCRBY leaderboard 100 "alice"       # 1600

# Count members in score range
ZCOUNT leaderboard 1000 2000

# Get members by score range
ZRANGEBYSCORE leaderboard 1000 2000

# Get number of members
ZCARD leaderboard

# Remove members
ZREM leaderboard "dave"`}
          />
        </section>

        <section>
          <h2>Key Commands</h2>
          <p>
            These commands work on keys regardless of the value type:
          </p>
          <CodeBlock
            title="Key operations"
            code={`# Check if key exists
EXISTS mykey

# Delete keys
DEL mykey
DEL key1 key2 key3

# Set expiration (seconds)
EXPIRE mykey 60

# Set expiration (milliseconds)
PEXPIRE mykey 60000

# Set expiration to a Unix timestamp
EXPIREAT mykey 1740700800

# Get remaining TTL
TTL mykey        # seconds
PTTL mykey       # milliseconds

# Remove expiration
PERSIST mykey

# Get key type
TYPE mykey

# Rename a key
RENAME oldkey newkey

# Find keys matching a pattern (use with caution)
KEYS "user:*"

# Iterate keys with cursor (production-safe)
SCAN 0 MATCH "user:*" COUNT 100`}
          />
        </section>
      </div>

      <DocsPager currentHref="/docs/data-types" />
    </div>
  );
}
