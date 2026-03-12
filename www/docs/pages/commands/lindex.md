# LINDEX

Works with list values.

## Syntax

```text
LINDEX <key> <index>
```

## Parameters

- `key`: Primary key to read or mutate.
- `index`: Zero-based position or JSON array index.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
LINDEX jobs 0
```

## BetterKV Notes

- Group: List.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
