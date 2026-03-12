# HSCAN

Works with hash fields stored under one key.

## Syntax

```text
HSCAN <key> <cursor> [MATCH <pattern>] [COUNT <count>]
```

## Parameters

- `key`: Primary key to read or mutate.
- `cursor`: Opaque incremental iteration cursor returned by a previous scan call.
- `pattern`: Glob-style match pattern.
- `count`: Requested number of items.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
HSCAN user:1 0 MATCH na* COUNT 50
```

## BetterKV Notes

- Group: Hash.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
