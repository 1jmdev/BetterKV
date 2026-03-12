# MGET

Works with string values and bitmap-style string operations.

## Syntax

```text
MGET <key> [key ...]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an array whose shape depends on the requested items and optional output flags.

## Examples

```bash
MGET user:1:name user:2:name user:3:name
```

## BetterKV Notes

- Group: String.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
