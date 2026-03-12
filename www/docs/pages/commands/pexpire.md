# PEXPIRE

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
PEXPIRE <key> <milliseconds> [NX | XX | GT | LT]
```

## Parameters

- `key`: Primary key to read or mutate.
- `milliseconds`: Command-specific `milliseconds` argument.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
PEXPIRE session:1 5000 XX
```

## BetterKV Notes

- Group: Keyspace.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
