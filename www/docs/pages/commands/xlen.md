# XLEN

Works with stream data structures and consumer groups.

## Syntax

```text
XLEN <key>
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns a stream-specific reply such as an entry id, entry list, pending summary, or consumer-group result.

## Examples

```bash
XLEN orders
```

## BetterKV Notes

- Group: Stream.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
