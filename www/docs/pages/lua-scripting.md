# Lua Scripting

BetterKV executes Lua 5.1 scripts atomically on the server side. Scripts run up to **9x faster** than in standard Redis due to a rewritten Lua execution engine.

## Why Server-Side Scripting?

- **Atomicity**: no other command runs while your script executes
- **Network efficiency**: one round trip instead of many
- **Complex logic**: conditionals, loops, multi-key operations

## EVAL — Run a Script

```bash
EVAL script numkeys [key [key ...]] [arg [arg ...]]
```

```bash
# Simple example: conditional set
EVAL "
  local val = redis.call('GET', KEYS[1])
  if val == ARGV[1] then
    return redis.call('SET', KEYS[1], ARGV[2])
  end
  return 0
" 1 mykey "old_value" "new_value"
```

Inside scripts:
- `KEYS[n]` — key arguments (1-indexed)
- `ARGV[n]` — value arguments (1-indexed)
- `redis.call(cmd, ...)` — execute a command (raises error on failure)
- `redis.pcall(cmd, ...)` — execute a command (returns error table on failure)

## EVALSHA — Run a Cached Script

Scripts are cached by their SHA1 hash. Load once, call many times:

```bash
# Load script, get SHA1
SCRIPT LOAD "return redis.call('GET', KEYS[1])"
# "e0e1f9fabfa9d353eca4c6f67b1e0ef68b5e4219"

# Execute by SHA1
EVALSHA e0e1f9fabfa9d353eca4c6f67b1e0ef68b5e4219 1 mykey
```

## Practical Examples

### Atomic Counter with Limit

```lua
-- Increment a counter, but cap it at a maximum value
local key = KEYS[1]
local max = tonumber(ARGV[1])

local current = tonumber(redis.call('GET', key) or 0)
if current >= max then
  return current
end

return redis.call('INCR', key)
```

```bash
EVAL "..." 1 counter:api_calls 1000
```

### Rate Limiter (Sliding Window)

```lua
local key = KEYS[1]
local limit = tonumber(ARGV[1])
local window = tonumber(ARGV[2])
local now = tonumber(ARGV[3])

-- Remove entries outside the window
redis.call('ZREMRANGEBYSCORE', key, 0, now - window)

-- Count current requests
local count = redis.call('ZCARD', key)

if count < limit then
  redis.call('ZADD', key, now, now)
  redis.call('EXPIRE', key, window)
  return 1  -- allowed
end

return 0  -- rate limited
```

### Distributed Lock (Redlock-style)

```lua
-- SET key value NX PX ttl (atomic acquire)
local result = redis.call('SET', KEYS[1], ARGV[1], 'NX', 'PX', ARGV[2])
if result then
  return 1  -- lock acquired
end
return 0  -- lock not acquired
```

Release lock safely:

```lua
-- Only delete if we own the lock
if redis.call('GET', KEYS[1]) == ARGV[1] then
  return redis.call('DEL', KEYS[1])
end
return 0
```

## Script Management

```bash
# Check if scripts are cached
SCRIPT EXISTS sha1 [sha1 ...]

# Flush all cached scripts
SCRIPT FLUSH

# Kill a running script (long-running only)
SCRIPT KILL
```

## Error Handling

```lua
-- redis.call raises an error on failure
local ok, err = pcall(function()
  return redis.call('GET', KEYS[1])
end)

if not ok then
  return redis.error_reply("Command failed: " .. err)
end

-- redis.pcall returns a table with err field on failure
local result = redis.pcall('HSET', KEYS[1])
if result.err then
  return redis.status_reply("error handled")
end
```

## Performance Tips

:::tip
BetterKV's Lua engine is 9x faster than Redis. To maximize throughput:

1. Use `EVALSHA` after the first call — avoids script parsing
2. Keep scripts short and focused — atomic execution blocks other commands
3. Avoid `redis.call('KEYS', '*')` inside scripts — O(N) over the entire keyspace
4. Use `ARGV` for values, `KEYS` for key names — enables cluster slot detection
:::

:::warning
Scripts longer than ~5ms may trigger the watchdog and be killed. For heavy computation, break it into smaller steps.
:::
