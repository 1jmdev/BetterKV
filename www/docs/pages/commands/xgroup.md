# XGROUP

Works with stream data structures and consumer groups.

## Syntax

```text
XGROUP <CREATE | CREATECONSUMER | DELCONSUMER | DESTROY | SETID> <key> <group> [arguments ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `group`: Consumer group name.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
XGROUP CREATE orders workers $ MKSTREAM
```

## BetterKV Notes

- Group: Stream.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
