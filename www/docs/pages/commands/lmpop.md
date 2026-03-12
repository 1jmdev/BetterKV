# LMPOP

Works with list values.

## Syntax

```text
LMPOP <numkeys> <key> [key ...] <LEFT | RIGHT> [COUNT <count>]
```

## Parameters

- `numkeys`: One or more keys processed by the command.
- `key`: Primary key to read or mutate.
- `count`: Requested number of items.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
LMPOP 2 pending processing LEFT COUNT 5
```

## BetterKV Notes

- Group: List.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
