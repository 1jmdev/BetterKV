# ZINTERSTORE

Works with sorted set values.

## Syntax

```text
ZINTERSTORE <destination> <numkeys> <key> [key ...] [WEIGHTS <weight ...>] [AGGREGATE <SUM | MIN | MAX>]
```

## Parameters

- `destination`: Key that receives the computed result.
- `numkeys`: One or more keys processed by the command.
- `key`: Primary key to read or mutate.
- `weight`: Command-specific `weight` argument.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
ZINTERSTORE combined 2 score:a score:b WEIGHTS 2 1
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
