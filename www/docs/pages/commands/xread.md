# XREAD

Works with stream data structures and consumer groups.

## Syntax

```text
XREAD [COUNT <count>] [BLOCK <ms>] STREAMS <key> [key ...] <id> [id ...]
```

## Parameters

- `count`: Requested number of items.
- `ms`: Command-specific `ms` argument.
- `key`: Primary key to read or mutate.
- `id`: Stream entry id.

## Returns

Returns a stream-specific reply such as an entry id, entry list, pending summary, or consumer-group result.

## Examples

```bash
XREAD COUNT 2 STREAMS orders 0-0
```

## BetterKV Notes

- Group: Stream.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
