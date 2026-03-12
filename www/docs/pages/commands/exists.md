# EXISTS

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
EXISTS <key> [key ...]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
EXISTS user:1 user:2 user:3
```

## BetterKV Notes

- Group: Keyspace.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
