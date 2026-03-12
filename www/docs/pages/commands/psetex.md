# PSETEX

Works with string values and bitmap-style string operations.

## Syntax

```text
PSETEX <key> <milliseconds> <value>
```

## Parameters

- `key`: Primary key to read or mutate.
- `milliseconds`: Command-specific `milliseconds` argument.
- `value`: Value written to BetterKV.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
PSETEX session:1 5000 active
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
