# GETEX

Works with string values and bitmap-style string operations.

## Syntax

```text
GETEX <key> [EX <seconds> | PX <milliseconds> | EXAT <unix-time-seconds> | PXAT <unix-time-ms> | PERSIST]
```

## Parameters

- `key`: Primary key to read or mutate.
- `seconds`: Command-specific `seconds` argument.
- `milliseconds`: Command-specific `milliseconds` argument.
- `unix`: Command-specific `unix` argument.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
GETEX session:1 EX 300
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
