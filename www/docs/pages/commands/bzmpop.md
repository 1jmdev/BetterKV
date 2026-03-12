# BZMPOP

Works with sorted set values.

## Syntax

```text
BZMPOP <timeout> <numkeys> <key> [key ...] <MIN | MAX> [COUNT <count>]
```

## Parameters

- `timeout`: Blocking timeout in seconds or milliseconds depending on the command.
- `numkeys`: One or more keys processed by the command.
- `key`: Primary key to read or mutate.
- `count`: Requested number of items.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
BZMPOP 5 2 board:a board:b MAX COUNT 2
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
