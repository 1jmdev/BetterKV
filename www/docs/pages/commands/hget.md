# HGET

Works with hash fields stored under one key.

## Syntax

```text
HGET <key> <field>
```

## Parameters

- `key`: Primary key to read or mutate.
- `field`: Hash field name.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
HGET user:1 name
```

## BetterKV Notes

- Group: Hash.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
