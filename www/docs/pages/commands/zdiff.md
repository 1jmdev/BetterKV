# ZDIFF

Works with sorted set values.

## Syntax

```text
ZDIFF <numkeys> <key> [key ...] [WITHSCORES]
```

## Parameters

- `numkeys`: One or more keys processed by the command.
- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
ZDIFF 2 board:a board:b WITHSCORES
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
