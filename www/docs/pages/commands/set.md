# SET

Works with string values and bitmap-style string operations.

## Syntax

```text
SET <key> <value> [NX | XX] [GET] [EX <seconds> | PX <milliseconds> | EXAT <unix-time-seconds> | PXAT <unix-time-ms> | KEEPTTL]
```

## Parameters

- `key`: Primary key to read or mutate.
- `value`: Value written to BetterKV.
- `seconds`: Command-specific `seconds` argument.
- `milliseconds`: Command-specific `milliseconds` argument.
- `unix`: Command-specific `unix` argument.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
SET session:1 "active" EX 60
SET feature:flag "on" NX
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
