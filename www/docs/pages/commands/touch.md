# TOUCH

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
TOUCH <key> [key ...]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
TOUCH user:1 user:2 user:3
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
