# ZREMRANGEBYLEX

Works with sorted set values.

## Syntax

```text
ZREMRANGEBYLEX <key> <min> <max>
```

## Parameters

- `key`: Primary key to read or mutate.
- `min`: Minimum score or lower lex/range bound.
- `max`: Maximum score or upper lex/range bound.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
ZREMRANGEBYLEX lexset [a [m
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
