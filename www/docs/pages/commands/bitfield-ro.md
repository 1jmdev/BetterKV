# BITFIELD_RO

Works with string values and bitmap-style string operations.

## Syntax

```text
BITFIELD_RO <key> GET <type> <offset> [GET <type> <offset> ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `type`: Type selector, encoding, or filter value used by the command.
- `offset`: Numeric offset used for bit, range, or paging behavior.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
BITFIELD_RO bitmap GET u8 0 GET u8 8
```

## BetterKV Notes

- Group: String.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
