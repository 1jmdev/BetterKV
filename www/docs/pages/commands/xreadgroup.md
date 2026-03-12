# XREADGROUP

Works with stream data structures and consumer groups.

## Syntax

```text
XREADGROUP GROUP <group> <consumer> [COUNT <count>] [BLOCK <ms>] [NOACK] STREAMS <key> [key ...] <id> [id ...]
```

## Parameters

- `group`: Consumer group name.
- `consumer`: Consumer name inside a stream group.
- `count`: Requested number of items.
- `ms`: Command-specific `ms` argument.
- `key`: Primary key to read or mutate.
- `id`: Stream entry id.

## Returns

Returns a stream-specific reply such as an entry id, entry list, pending summary, or consumer-group result.

## Examples

```bash
XREADGROUP GROUP workers consumer-1 COUNT 10 STREAMS orders >
```

## BetterKV Notes

- Group: Stream.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
