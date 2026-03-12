# BLMPOP

Works with list values.

## Syntax

```text
BLMPOP <timeout> <numkeys> <key> [key ...] <LEFT | RIGHT> [COUNT <count>]
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
BLMPOP 5 2 jobs delayed LEFT COUNT 10
```

## BetterKV Notes

- Group: List.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
