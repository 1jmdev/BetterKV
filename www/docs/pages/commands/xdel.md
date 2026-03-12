# XDEL

Works with stream data structures and consumer groups.

## Syntax

```text
XDEL <key> <id> [id ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `id`: Stream entry id.

## Returns

Returns a stream-specific reply such as an entry id, entry list, pending summary, or consumer-group result.

## Examples

```bash
XDEL orders 1710000000000-0
```

## BetterKV Notes

- Group: Stream.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
