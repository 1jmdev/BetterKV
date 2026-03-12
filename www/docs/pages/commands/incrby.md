# INCRBY

Works with string values and bitmap-style string operations.

## Syntax

```text
INCRBY <key> <increment>
```

## Parameters

- `key`: Primary key to read or mutate.
- `increment`: Numeric delta applied atomically to the current value.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
INCRBY stats:requests 100
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
