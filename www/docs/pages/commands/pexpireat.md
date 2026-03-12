# PEXPIREAT

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
PEXPIREAT <key> <unix-time-ms> [NX | XX | GT | LT]
```

## Parameters

- `key`: Primary key to read or mutate.
- `unix`: Command-specific `unix` argument.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
PEXPIREAT session:1 1767225600000
```

## BetterKV Notes

- Group: Keyspace.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
