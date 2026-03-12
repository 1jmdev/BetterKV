# SORT

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
SORT <key> [BY <pattern>] [LIMIT <offset> <count>] [GET <pattern> [GET <pattern> ...]] [ASC | DESC] [ALPHA] [STORE <destination>]
```

## Parameters

- `key`: Primary key to read or mutate.
- `pattern`: Glob-style match pattern.
- `offset`: Numeric offset used for bit, range, or paging behavior.
- `count`: Requested number of items.
- `destination`: Key that receives the computed result.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
SORT leaderboard DESC LIMIT 0 10
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
