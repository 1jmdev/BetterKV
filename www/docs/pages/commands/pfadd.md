# PFADD

Works with string values and bitmap-style string operations.

## Syntax

```text
PFADD <key> <element> [element ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `element`: List element value.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
PFADD visitors:today user:1 user:2 user:3
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
