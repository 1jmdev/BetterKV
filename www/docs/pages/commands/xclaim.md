# XCLAIM

Works with stream data structures and consumer groups.

## Syntax

```text
XCLAIM <key> <group> <consumer> <min-idle-time-ms> <id> [id ...] [IDLE <ms>] [TIME <unix-ms>] [RETRYCOUNT <count>] [FORCE] [JUSTID]
```

## Parameters

- `key`: Primary key to read or mutate.
- `group`: Consumer group name.
- `consumer`: Consumer name inside a stream group.
- `min`: Minimum score or lower lex/range bound.
- `id`: Stream entry id.
- `ms`: Command-specific `ms` argument.
- `unix`: Command-specific `unix` argument.
- `count`: Requested number of items.

## Returns

Returns a stream-specific reply such as an entry id, entry list, pending summary, or consumer-group result.

## Examples

```bash
XCLAIM orders workers consumer-2 60000 1710000000000-0
```

## BetterKV Notes

- Group: Stream.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
