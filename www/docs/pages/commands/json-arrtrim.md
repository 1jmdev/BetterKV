# JSON.ARRTRIM

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.ARRTRIM <key> <path> <start> <stop>
```

## Parameters

- `key`: Primary key to read or mutate.
- `path`: JSONPath-like location inside the stored JSON document.
- `start`: Start position, range boundary, or stream id lower bound.
- `stop`: End position or range boundary.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
JSON.ARRTRIM profile $.items 0 9
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
