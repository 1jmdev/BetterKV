# ZUNION

Works with sorted set values.

## Syntax

```text
ZUNION <numkeys> <key> [key ...] [WEIGHTS <weight ...>] [AGGREGATE <SUM | MIN | MAX>] [WITHSCORES]
```

## Parameters

- `numkeys`: One or more keys processed by the command.
- `key`: Primary key to read or mutate.
- `weight`: Command-specific `weight` argument.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
ZUNION 2 score:a score:b WEIGHTS 2 1
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
