# ZINCRBY

Works with sorted set values.

## Syntax

```text
ZINCRBY <key> <increment> <member>
```

## Parameters

- `key`: Primary key to read or mutate.
- `increment`: Numeric delta applied atomically to the current value.
- `member`: Set, sorted-set, or geo member.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
ZINCRBY leaderboard 5 alice
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
