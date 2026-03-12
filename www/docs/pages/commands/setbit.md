# SETBIT

Works with string values and bitmap-style string operations.

## Syntax

```text
SETBIT <key> <offset> <0 | 1>
```

## Parameters

- `key`: Primary key to read or mutate.
- `offset`: Numeric offset used for bit, range, or paging behavior.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
SETBIT bitmap 7 1
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
