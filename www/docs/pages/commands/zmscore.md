# ZMSCORE

Works with sorted set values.

## Syntax

```text
ZMSCORE <key> <member> [member ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `member`: Set, sorted-set, or geo member.

## Returns

Returns an array whose shape depends on the requested items and optional output flags.

## Examples

```bash
ZMSCORE leaderboard alice bob carol
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
