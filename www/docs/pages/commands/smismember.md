# SMISMEMBER

Works with set values.

## Syntax

```text
SMISMEMBER <key> <member> [member ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `member`: Set, sorted-set, or geo member.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
SMISMEMBER tags rust redis lua
```

## BetterKV Notes

- Group: Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
