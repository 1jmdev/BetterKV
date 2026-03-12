# JSON.RESP

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.RESP <key> [path]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
JSON.RESP profile $
```

## BetterKV Notes

- Group: Keyspace.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
