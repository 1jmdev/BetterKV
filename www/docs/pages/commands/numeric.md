# Numeric Commands

BetterKV stores numbers as strings but provides atomic integer and float arithmetic.

## INCR / DECR

```
INCR key
DECR key
```

Increment or decrement the integer value of `key` by 1. If key doesn't exist, it starts at 0. Errors if value is not an integer.

**Complexity:** O(1)

```bash
SET visits 100
INCR visits    # 101
INCR visits    # 102
DECR visits    # 101

# Key doesn't exist — starts at 0
DEL new_counter
INCR new_counter   # 1
```

## INCRBY / DECRBY

```
INCRBY key increment
DECRBY key decrement
```

Increment or decrement by a specific integer.

**Complexity:** O(1)

```bash
SET score 1000
INCRBY score 500     # 1500
INCRBY score -100    # 1400
DECRBY score 200     # 1200
```

## INCRBYFLOAT

```
INCRBYFLOAT key increment
```

Increment by a floating-point number. Supports scientific notation. Result is stored with up to 17 significant digits.

**Complexity:** O(1)

```bash
SET price 10.50
INCRBYFLOAT price 0.25    # "10.75"
INCRBYFLOAT price -1.50   # "9.25"
INCRBYFLOAT price 1e2     # "109.25"
```

:::tip
Results are stored as strings. `INCRBYFLOAT` uses IEEE 754 double precision. For financial calculations requiring exact decimal arithmetic, store values as integers (e.g., cents instead of dollars).
:::

## Pattern: Rate Limiting

Use `INCR` with `EXPIRE` for atomic rate limiting:

```lua
-- Lua script: rate limit with sliding window counter
local key = KEYS[1]
local limit = tonumber(ARGV[1])
local window = tonumber(ARGV[2])

local current = redis.call('INCR', key)
if current == 1 then
  redis.call('EXPIRE', key, window)
end

if current > limit then
  return 0  -- rate limited
end
return 1  -- allowed
```

```bash
# Allow 100 requests per 60 seconds
EVAL "..." 1 rate:user:123 100 60
```

## Pattern: Atomic Counter with Reset

```bash
# Get current value and reset to 0 atomically
GETSET counter 0
```

## Range Limits

| Type | Min | Max |
|------|-----|-----|
| Integer | -9,223,372,036,854,775,808 | 9,223,372,036,854,775,807 |
| Float | ±5×10⁻³²⁴ | ±1.8×10³⁰⁸ |

Attempting to exceed these limits returns an overflow error.
