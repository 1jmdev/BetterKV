# ZRANGEBYSCORE

Works with sorted set values.

## Syntax

```text
ZRANGEBYSCORE <key> <min> <max> [WITHSCORES] [LIMIT <offset> <count>]
```

## Parameters

- `key`: Primary key to read or mutate.
- `min`: Minimum score or lower lex/range bound.
- `max`: Maximum score or upper lex/range bound.
- `offset`: Numeric offset used for bit, range, or paging behavior.
- `count`: Requested number of items.

## Returns

Returns an array whose shape depends on the requested items and optional output flags.

## Examples

```bash
ZRANGEBYSCORE leaderboard 50 100 WITHSCORES LIMIT 0 20
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
