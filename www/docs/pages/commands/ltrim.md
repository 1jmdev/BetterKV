# LTRIM

Works with list values.

## Syntax

```text
LTRIM <key> <start> <stop>
```

## Parameters

- `key`: Primary key to read or mutate.
- `start`: Start position, range boundary, or stream id lower bound.
- `stop`: End position or range boundary.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
LTRIM recent:events 0 99
```

## BetterKV Notes

- Group: List.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
