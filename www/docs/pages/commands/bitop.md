# BITOP

Works with string values and bitmap-style string operations.

## Syntax

```text
BITOP <AND | OR | XOR | NOT> <destkey> <key> [key ...]
```

## Parameters

- `destkey`: Key that receives the computed result.
- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
BITOP OR combined flags:a flags:b
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
