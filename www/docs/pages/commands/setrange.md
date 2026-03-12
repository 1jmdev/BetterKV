# SETRANGE

Works with string values and bitmap-style string operations.

## Syntax

```text
SETRANGE <key> <offset> <value>
```

## Parameters

- `key`: Primary key to read or mutate.
- `offset`: Numeric offset used for bit, range, or paging behavior.
- `value`: Value written to BetterKV.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
SETRANGE title 6 BetterKV
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
