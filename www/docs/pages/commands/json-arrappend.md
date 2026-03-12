# JSON.ARRAPPEND

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.ARRAPPEND <key> [path] <value> [value ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `value`: Value written to BetterKV.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
JSON.ARRAPPEND profile $.items "blue" "green"
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
