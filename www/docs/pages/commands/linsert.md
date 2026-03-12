# LINSERT

Works with list values.

## Syntax

```text
LINSERT <key> <BEFORE | AFTER> <pivot> <element>
```

## Parameters

- `key`: Primary key to read or mutate.
- `pivot`: Existing list element used as the insertion anchor.
- `element`: List element value.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
LINSERT jobs BEFORE job-2 urgent-job
```

## BetterKV Notes

- Group: List.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
