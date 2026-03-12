# JSON.MGET

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.MGET <key> [key ...] <path>
```

## Parameters

- `key`: Primary key to read or mutate.
- `path`: JSONPath-like location inside the stored JSON document.

## Returns

Returns an array whose shape depends on the requested items and optional output flags.

## Examples

```bash
JSON.MGET user:1 user:2 $.name
```

## BetterKV Notes

- Group: Keyspace.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
