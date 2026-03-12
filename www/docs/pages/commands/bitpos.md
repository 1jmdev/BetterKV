# BITPOS

Works with string values and bitmap-style string operations.

## Syntax

```text
BITPOS <key> <bit> [start [end [BYTE | BIT]]]
```

## Parameters

- `key`: Primary key to read or mutate.
- `bit`: Bit value, usually `0` or `1`.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
BITPOS bitmap 1 0 -1 BYTE
```

## BetterKV Notes

- Group: String.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
