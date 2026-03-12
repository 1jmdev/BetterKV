# JSON.ARRLEN

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.ARRLEN <key> [path]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
JSON.ARRLEN profile $.items
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
