# JSON.NUMINCRBY

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.NUMINCRBY <key> <path> <number>
```

## Parameters

- `key`: Primary key to read or mutate.
- `path`: JSONPath-like location inside the stored JSON document.
- `number`: Command-specific `number` argument.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
JSON.NUMINCRBY profile $.score 10
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
