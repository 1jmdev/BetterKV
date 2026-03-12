# SCAN

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
SCAN <cursor> [MATCH <pattern>] [COUNT <count>] [TYPE <type>]
```

## Parameters

- `cursor`: Opaque incremental iteration cursor returned by a previous scan call.
- `pattern`: Glob-style match pattern.
- `count`: Requested number of items.
- `type`: Type selector, encoding, or filter value used by the command.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
SCAN 0 MATCH user:* COUNT 100
```

## BetterKV Notes

- Group: Keyspace.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
