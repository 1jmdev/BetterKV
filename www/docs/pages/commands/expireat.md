# EXPIREAT

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
EXPIREAT <key> <unix-time-seconds> [NX | XX | GT | LT]
```

## Parameters

- `key`: Primary key to read or mutate.
- `unix`: Command-specific `unix` argument.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
EXPIREAT session:1 1767225600
```

## BetterKV Notes

- Group: Keyspace.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
