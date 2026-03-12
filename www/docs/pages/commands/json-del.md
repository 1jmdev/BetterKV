# JSON.DEL

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.DEL <key> [path]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
JSON.DEL profile $.temporary
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
