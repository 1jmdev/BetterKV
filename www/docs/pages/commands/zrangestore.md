# ZRANGESTORE

Works with sorted set values.

## Syntax

```text
ZRANGESTORE <destination> <source> <start> <stop> [BYSCORE | BYLEX] [REV] [LIMIT <offset> <count>]
```

## Parameters

- `destination`: Key that receives the computed result.
- `source`: Source key.
- `start`: Start position, range boundary, or stream id lower bound.
- `stop`: End position or range boundary.
- `offset`: Numeric offset used for bit, range, or paging behavior.
- `count`: Requested number of items.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
ZRANGESTORE top:10 leaderboard 0 9 REV
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
