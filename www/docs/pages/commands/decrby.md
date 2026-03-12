# DECRBY

Works with string values and bitmap-style string operations.

## Syntax

```text
DECRBY <key> <decrement>
```

## Parameters

- `key`: Primary key to read or mutate.
- `decrement`: Numeric amount subtracted atomically from the current value.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
DECRBY inventory:sku:1 5
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
