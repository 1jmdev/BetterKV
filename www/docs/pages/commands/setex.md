# SETEX

Works with string values and bitmap-style string operations.

## Syntax

```text
SETEX <key> <seconds> <value>
```

## Parameters

- `key`: Primary key to read or mutate.
- `seconds`: Command-specific `seconds` argument.
- `value`: Value written to BetterKV.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
SETEX cache:home 60 "rendered page"
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
