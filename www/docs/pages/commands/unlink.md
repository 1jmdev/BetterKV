# UNLINK

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
UNLINK <key> [key ...]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
UNLINK temp:1 temp:2
```

## BetterKV Notes

- Group: Keyspace.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
