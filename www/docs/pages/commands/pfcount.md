# PFCOUNT

Works with string values and bitmap-style string operations.

## Syntax

```text
PFCOUNT <key> [key ...]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
PFCOUNT visitors:today visitors:yesterday
```

## BetterKV Notes

- Group: String.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
