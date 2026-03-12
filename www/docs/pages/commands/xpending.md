# XPENDING

Works with stream data structures and consumer groups.

## Syntax

```text
XPENDING <key> <group> [start end count [consumer]]
```

## Parameters

- `key`: Primary key to read or mutate.
- `group`: Consumer group name.

## Returns

Returns a stream-specific reply such as an entry id, entry list, pending summary, or consumer-group result.

## Examples

```bash
XPENDING orders workers - + 10 consumer-1
```

## BetterKV Notes

- Group: Stream.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
