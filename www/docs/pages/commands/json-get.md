# JSON.GET

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.GET <key> [path [path ...]]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
JSON.GET profile $.name $.email
```

## BetterKV Notes

- Group: Keyspace.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
