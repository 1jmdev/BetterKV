# MOVE

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
MOVE <key> <db>
```

## Parameters

- `key`: Primary key to read or mutate.
- `db`: Logical database index.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
MOVE temp:key 1
```

## BetterKV Notes

- Group: Keyspace.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
