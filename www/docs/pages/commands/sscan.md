# SSCAN

Works with set values.

## Syntax

```text
SSCAN <key> <cursor> [MATCH <pattern>] [COUNT <count>]
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
SSCAN tags 0 MATCH prod:* COUNT 100
```

## BetterKV Notes

- Group: Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
