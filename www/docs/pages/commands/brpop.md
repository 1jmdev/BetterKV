# BRPOP

Works with list values.

## Syntax

```text
BRPOP <key> [key ...] <timeout>
```

## Parameters

- `key`: Primary key to read or mutate.
- `timeout`: Blocking timeout in seconds or milliseconds depending on the command.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
BRPOP jobs delayed 5
```

## BetterKV Notes

- Group: List.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
