# GETRANGE

Works with string values and bitmap-style string operations.

## Syntax

```text
GETRANGE <key> <start> <end>
```

## Parameters

- `key`: Primary key to read or mutate.
- `start`: Start position, range boundary, or stream id lower bound.
- `end`: End position or range boundary.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
GETRANGE article:1 0 127
```

## BetterKV Notes

- Group: String.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
