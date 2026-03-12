# LRANGE

Works with list values.

## Syntax

```text
LRANGE <key> <start> <stop>
```

## Parameters

- `key`: Primary key to read or mutate.
- `start`: Start position, range boundary, or stream id lower bound.
- `stop`: End position or range boundary.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
LRANGE jobs 0 9
```

## BetterKV Notes

- Group: List.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
