# ZREVRANGEBYSCORE

Works with sorted set values.

## Syntax

```text
ZREVRANGEBYSCORE <key> <max> <min> [WITHSCORES] [LIMIT <offset> <count>]
```

## Parameters

- `key`: Primary key to read or mutate.
- `max`: Maximum score or upper lex/range bound.
- `min`: Minimum score or lower lex/range bound.
- `offset`: Numeric offset used for bit, range, or paging behavior.
- `count`: Requested number of items.

## Returns

Returns an array whose shape depends on the requested items and optional output flags.

## Examples

```bash
ZREVRANGEBYSCORE leaderboard 100 50 WITHSCORES LIMIT 0 20
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
