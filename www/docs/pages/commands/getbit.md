# GETBIT

Works with string values and bitmap-style string operations.

## Syntax

```text
GETBIT <key> <offset>
```

## Parameters

- `key`: Primary key to read or mutate.
- `offset`: Numeric offset used for bit, range, or paging behavior.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
GETBIT bitmap 42
```

## BetterKV Notes

- Group: String.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
