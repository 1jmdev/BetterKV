# HMGET

Works with hash fields stored under one key.

## Syntax

```text
HMGET <key> <field> [field ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `field`: Hash field name.

## Returns

Returns an array whose shape depends on the requested items and optional output flags.

## Examples

```bash
HMGET user:1 name email
```

## BetterKV Notes

- Group: Hash.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
