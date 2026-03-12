# JSON.TYPE

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.TYPE <key> [path]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
JSON.TYPE profile $.name
```

## BetterKV Notes

- Group: Keyspace.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
