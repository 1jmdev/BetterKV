# ZDIFFSTORE

Works with sorted set values.

## Syntax

```text
ZDIFFSTORE <destination> <numkeys> <key> [key ...]
```

## Parameters

- `destination`: Key that receives the computed result.
- `numkeys`: One or more keys processed by the command.
- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
ZDIFFSTORE only:a 2 score:a score:b
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
