# DUMP

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
DUMP <key>
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
DUMP ...
```

## BetterKV Notes

- Group: Keyspace.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
