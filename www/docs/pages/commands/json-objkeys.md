# JSON.OBJKEYS

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.OBJKEYS <key> [path]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
JSON.OBJKEYS profile $.flags
```

## BetterKV Notes

- Group: Keyspace.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
