# ZADD

Works with sorted set values.

## Syntax

```text
ZADD <key> [NX | XX] [GT | LT] [CH] [INCR] <score> <member> [score member ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `score`: Sorted-set score value.
- `member`: Set, sorted-set, or geo member.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
ZADD leaderboard 120 alice 98 bob
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
