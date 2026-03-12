# ZMPOP

Works with sorted set values.

## Syntax

```text
ZMPOP <numkeys> <key> [key ...] <MIN | MAX> [COUNT <count>]
```

## Parameters

- `numkeys`: One or more keys processed by the command.
- `key`: Primary key to read or mutate.
- `count`: Requested number of items.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
ZMPOP 2 set:a set:b MIN COUNT 3
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
