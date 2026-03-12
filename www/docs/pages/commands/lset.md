# LSET

Works with list values.

## Syntax

```text
LSET <key> <index> <element>
```

## Parameters

- `key`: Primary key to read or mutate.
- `index`: Zero-based position or JSON array index.
- `element`: List element value.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
LSET jobs 0 urgent-job
```

## BetterKV Notes

- Group: List.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
