# ZPOPMIN

Works with sorted set values.

## Syntax

```text
ZPOPMIN <key> [count]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
ZPOPMIN leaderboard 2
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
