# BITFIELD

Works with string values and bitmap-style string operations.

## Syntax

```text
BITFIELD <key> [GET <type> <offset>] [SET <type> <offset> <value>] [INCRBY <type> <offset> <increment>] [OVERFLOW WRAP | SAT | FAIL]
```

## Parameters

- `key`: Primary key to read or mutate.
- `type`: Type selector, encoding, or filter value used by the command.
- `offset`: Numeric offset used for bit, range, or paging behavior.
- `value`: Value written to BetterKV.
- `increment`: Numeric delta applied atomically to the current value.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
BITFIELD ...
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
