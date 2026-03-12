# XTRIM

Works with stream data structures and consumer groups.

## Syntax

```text
XTRIM <key> MAXLEN | MINID [= | ~] <threshold> [LIMIT <count>]
```

## Parameters

- `key`: Primary key to read or mutate.
- `threshold`: Trimming threshold, score bound, or command-specific numeric limit.
- `count`: Requested number of items.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
XTRIM orders MAXLEN ~ 10000
```

## BetterKV Notes

- Group: Stream.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
