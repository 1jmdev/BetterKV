# BITCOUNT

Works with string values and bitmap-style string operations.

## Syntax

```text
BITCOUNT <key> [start end [BYTE | BIT]]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
BITCOUNT ...
```

## BetterKV Notes

- Group: String.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
