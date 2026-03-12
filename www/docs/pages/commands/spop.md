# SPOP

Works with set values.

## Syntax

```text
SPOP <key> [count]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
SPOP tags 2
```

## BetterKV Notes

- Group: Set.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
