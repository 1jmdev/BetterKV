# HKEYS

Works with hash fields stored under one key.

## Syntax

```text
HKEYS <key>
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an array whose shape depends on the requested items and optional output flags.

## Examples

```bash
HKEYS user:1
```

## BetterKV Notes

- Group: Hash.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
