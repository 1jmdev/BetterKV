# SINTERCARD

Works with set values.

## Syntax

```text
SINTERCARD <numkeys> <key> [key ...] [LIMIT <limit>]
```

## Parameters

- `numkeys`: One or more keys processed by the command.
- `key`: Primary key to read or mutate.
- `limit`: Command-specific `limit` argument.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
SINTERCARD 3 tags:a tags:b tags:c LIMIT 100
```

## BetterKV Notes

- Group: Set.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
