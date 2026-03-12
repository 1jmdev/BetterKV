# XAUTOCLAIM

Works with stream data structures and consumer groups.

## Syntax

```text
XAUTOCLAIM <key> <group> <consumer> <min-idle-time-ms> <start> [COUNT <count>] [JUSTID]
```

## Parameters

- `key`: Primary key to read or mutate.
- `group`: Consumer group name.
- `consumer`: Consumer name inside a stream group.
- `min`: Minimum score or lower lex/range bound.
- `start`: Start position, range boundary, or stream id lower bound.
- `count`: Requested number of items.

## Returns

Returns a stream-specific reply such as an entry id, entry list, pending summary, or consumer-group result.

## Examples

```bash
XAUTOCLAIM orders workers consumer-2 60000 0-0 COUNT 10
```

## BetterKV Notes

- Group: Stream.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
