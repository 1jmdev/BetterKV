# ZSCAN

Works with sorted set values.

## Syntax

```text
ZSCAN <key> <cursor> [MATCH <pattern>] [COUNT <count>]
```

## Parameters

- `key`: Primary key to read or mutate.
- `cursor`: Opaque incremental iteration cursor returned by a previous scan call.
- `pattern`: Glob-style match pattern.
- `count`: Requested number of items.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
ZSCAN leaderboard 0 MATCH a* COUNT 50
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
