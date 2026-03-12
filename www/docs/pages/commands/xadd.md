# XADD

Works with stream data structures and consumer groups.

## Syntax

```text
XADD <key> [NOMKSTREAM] [MAXLEN | MINID [= | ~] <threshold> [LIMIT <count>]] <id | *> <field> <value> [field value ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `threshold`: Trimming threshold, score bound, or command-specific numeric limit.
- `count`: Requested number of items.
- `id`: Stream entry id.
- `*`: Command-specific `*` argument.
- `field`: Hash field name.
- `value`: Value written to BetterKV.

## Returns

Returns a stream-specific reply such as an entry id, entry list, pending summary, or consumer-group result.

## Examples

```bash
XADD orders * status pending total 42.50
```

## BetterKV Notes

- Group: Stream.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
