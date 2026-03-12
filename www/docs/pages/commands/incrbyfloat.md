# INCRBYFLOAT

Works with string values and bitmap-style string operations.

## Syntax

```text
INCRBYFLOAT <key> <increment>
```

## Parameters

- `key`: Primary key to read or mutate.
- `increment`: Numeric delta applied atomically to the current value.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
INCRBYFLOAT price 0.75
```

## BetterKV Notes

- Group: String.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
