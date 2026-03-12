# XRANGE

Works with stream data structures and consumer groups.

## Syntax

```text
XRANGE <key> <start> <end> [COUNT <count>]
```

## Parameters

- `key`: Primary key to read or mutate.
- `start`: Start position, range boundary, or stream id lower bound.
- `end`: End position or range boundary.
- `count`: Requested number of items.

## Returns

Returns a stream-specific reply such as an entry id, entry list, pending summary, or consumer-group result.

## Examples

```bash
XRANGE orders - + COUNT 10
```

## BetterKV Notes

- Group: Stream.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
