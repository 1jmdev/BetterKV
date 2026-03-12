# ZRANDMEMBER

Works with sorted set values.

## Syntax

```text
ZRANDMEMBER <key> [count [WITHSCORES]]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an array whose shape depends on the requested items and optional output flags.

## Examples

```bash
ZRANDMEMBER leaderboard 3 WITHSCORES
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
