# DEL

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
DEL <key> [key ...]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
DEL cache:a cache:b cache:c
```

## BetterKV Notes

- Group: Keyspace.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
