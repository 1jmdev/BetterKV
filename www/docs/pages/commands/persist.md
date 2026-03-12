# PERSIST

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
PERSIST <key>
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
PERSIST session:1
```

## BetterKV Notes

- Group: Keyspace.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
