# DECR

Works with string values and bitmap-style string operations.

## Syntax

```text
DECR <key>
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
DECR counters:jobs
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
