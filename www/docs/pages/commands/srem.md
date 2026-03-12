# SREM

Works with set values.

## Syntax

```text
SREM <key> <member> [member ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `member`: Set, sorted-set, or geo member.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
SREM tags redis
```

## BetterKV Notes

- Group: Set.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
