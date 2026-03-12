# RPUSH

Works with list values.

## Syntax

```text
RPUSH <key> <element> [element ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `element`: List element value.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
RPUSH jobs job-1 job-2 job-3
```

## BetterKV Notes

- Group: List.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
