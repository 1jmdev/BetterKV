# RPOP

Works with list values.

## Syntax

```text
RPOP <key> [count]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
RPOP jobs
```

## BetterKV Notes

- Group: List.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
