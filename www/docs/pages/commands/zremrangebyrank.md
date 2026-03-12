# ZREMRANGEBYRANK

Works with sorted set values.

## Syntax

```text
ZREMRANGEBYRANK <key> <start> <stop>
```

## Parameters

- `key`: Primary key to read or mutate.
- `start`: Start position, range boundary, or stream id lower bound.
- `stop`: End position or range boundary.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
ZREMRANGEBYRANK leaderboard 0 9
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
