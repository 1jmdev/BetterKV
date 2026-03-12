# ZREVRANGE

Works with sorted set values.

## Syntax

```text
ZREVRANGE <key> <start> <stop> [WITHSCORES]
```

## Parameters

- `key`: Primary key to read or mutate.
- `start`: Start position, range boundary, or stream id lower bound.
- `stop`: End position or range boundary.

## Returns

Returns an array whose shape depends on the requested items and optional output flags.

## Examples

```bash
ZREVRANGE leaderboard 0 9 WITHSCORES
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
