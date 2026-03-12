# JSON.ARRINSERT

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.ARRINSERT <key> <path> <index> <value> [value ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `path`: JSONPath-like location inside the stored JSON document.
- `index`: Zero-based position or JSON array index.
- `value`: Value written to BetterKV.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
JSON.ARRINSERT profile $.items 0 "first"
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
