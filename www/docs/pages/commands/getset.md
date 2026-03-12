# GETSET

Works with string values and bitmap-style string operations.

## Syntax

```text
GETSET <key> <value>
```

## Parameters

- `key`: Primary key to read or mutate.
- `value`: Value written to BetterKV.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
GETSET release "2026.03"
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
