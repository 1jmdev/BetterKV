# XACK

Works with stream data structures and consumer groups.

## Syntax

```text
XACK <key> <group> <id> [id ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `group`: Consumer group name.
- `id`: Stream entry id.

## Returns

Returns a stream-specific reply such as an entry id, entry list, pending summary, or consumer-group result.

## Examples

```bash
XACK orders processing 1710000000000-0
```

## BetterKV Notes

- Group: Stream.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
