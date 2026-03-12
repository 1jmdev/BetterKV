# ZREM

Works with sorted set values.

## Syntax

```text
ZREM <key> <member> [member ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `member`: Set, sorted-set, or geo member.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
ZREM leaderboard alice
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
