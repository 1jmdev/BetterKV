# LPUSH

Works with list values.

## Syntax

```text
LPUSH <key> <element> [element ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `element`: List element value.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
LPUSH jobs job-3 job-2 job-1
```

## BetterKV Notes

- Group: List.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
