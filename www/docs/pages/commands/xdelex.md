# XDELEX

Works with stream data structures and consumer groups.

## Syntax

```text
XDELEX <key> <id> [id ...] [MAXLEN <count>]
```

## Parameters

- `key`: Primary key to read or mutate.
- `id`: Stream entry id.
- `count`: Requested number of items.

## Returns

Returns a stream-specific reply such as an entry id, entry list, pending summary, or consumer-group result.

## Examples

```bash
XDELEX orders 1710000000000-0 1710000000001-0
```

## BetterKV Notes

- Group: Stream.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
