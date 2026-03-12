# ZRANGE

Works with sorted set values.

## Syntax

```text
ZRANGE <key> <start> <stop> [BYSCORE | BYLEX] [REV] [LIMIT <offset> <count>] [WITHSCORES]
```

## Parameters

- `key`: Primary key to read or mutate.
- `start`: Start position, range boundary, or stream id lower bound.
- `stop`: End position or range boundary.
- `offset`: Numeric offset used for bit, range, or paging behavior.
- `count`: Requested number of items.

## Returns

Returns an array whose shape depends on the requested items and optional output flags.

## Examples

```bash
ZRANGE leaderboard 0 9 WITHSCORES
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
