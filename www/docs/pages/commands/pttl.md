# PTTL

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
PTTL <key>
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns the remaining time to live for the key, using Redis-compatible sentinel values when no expiry or key exists.

## Examples

```bash
PTTL session:1
```

## BetterKV Notes

- Group: Keyspace.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
