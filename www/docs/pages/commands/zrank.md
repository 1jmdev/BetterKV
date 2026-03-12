# ZRANK

Works with sorted set values.

## Syntax

```text
ZRANK <key> <member> [WITHSCORE]
```

## Parameters

- `key`: Primary key to read or mutate.
- `member`: Set, sorted-set, or geo member.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
ZRANK leaderboard alice WITHSCORE
```

## BetterKV Notes

- Group: Sorted Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
