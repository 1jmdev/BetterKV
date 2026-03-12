# HINCRBY

Works with hash fields stored under one key.

## Syntax

```text
HINCRBY <key> <field> <increment>
```

## Parameters

- `key`: Primary key to read or mutate.
- `field`: Hash field name.
- `increment`: Numeric delta applied atomically to the current value.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
HINCRBY counters views 1
```

## BetterKV Notes

- Group: Hash.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
