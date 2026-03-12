# JSON.STRLEN

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.STRLEN <key> [path]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
JSON.STRLEN profile $.name
```

## BetterKV Notes

- Group: Keyspace.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
