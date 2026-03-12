# ZSCORE

Works with sorted set values.

## Syntax

```text
ZSCORE <key> <member>
```

## Parameters

- `key`: Primary key to read or mutate.
- `member`: Set, sorted-set, or geo member.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
ZSCORE leaderboard alice
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
