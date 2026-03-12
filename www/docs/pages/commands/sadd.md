# SADD

Works with set values.

## Syntax

```text
SADD <key> <member> [member ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `member`: Set, sorted-set, or geo member.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
SADD tags rust redis low-latency
```

## BetterKV Notes

- Group: Set.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
