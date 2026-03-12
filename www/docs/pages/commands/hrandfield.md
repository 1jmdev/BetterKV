# HRANDFIELD

Works with hash fields stored under one key.

## Syntax

```text
HRANDFIELD <key> [count [WITHVALUES]]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
HRANDFIELD user:1 2 WITHVALUES
```

## BetterKV Notes

- Group: Hash.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
