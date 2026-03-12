# XREVRANGE

Works with stream data structures and consumer groups.

## Syntax

```text
XREVRANGE <key> <end> <start> [COUNT <count>]
```

## Parameters

- `key`: Primary key to read or mutate.
- `end`: End position or range boundary.
- `start`: Start position, range boundary, or stream id lower bound.
- `count`: Requested number of items.

## Returns

Returns a stream-specific reply such as an entry id, entry list, pending summary, or consumer-group result.

## Examples

```bash
XREVRANGE orders + - COUNT 10
```

## BetterKV Notes

- Group: Stream.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
