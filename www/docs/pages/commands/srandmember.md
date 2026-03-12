# SRANDMEMBER

Works with set values.

## Syntax

```text
SRANDMEMBER <key> [count]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
SRANDMEMBER tags 3
```

## BetterKV Notes

- Group: Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
