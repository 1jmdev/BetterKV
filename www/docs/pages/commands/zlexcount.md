# ZLEXCOUNT

Works with sorted set values.

## Syntax

```text
ZLEXCOUNT <key> <min> <max>
```

## Parameters

- `key`: Primary key to read or mutate.
- `min`: Minimum score or lower lex/range bound.
- `max`: Maximum score or upper lex/range bound.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
ZLEXCOUNT lexset [a [z
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
