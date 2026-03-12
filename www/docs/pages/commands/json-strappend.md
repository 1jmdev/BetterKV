# JSON.STRAPPEND

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.STRAPPEND <key> [path] <value> [value ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `value`: Value written to BetterKV.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
JSON.STRAPPEND profile $.name " Lovelace"
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
