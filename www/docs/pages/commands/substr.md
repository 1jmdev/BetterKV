# SUBSTR

Works with string values and bitmap-style string operations.

## Syntax

```text
SUBSTR <key> <start> <end>
```

## Parameters

- `key`: Primary key to read or mutate.
- `start`: Start position, range boundary, or stream id lower bound.
- `end`: End position or range boundary.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
SUBSTR title 0 4
```

## BetterKV Notes

- Group: String.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
