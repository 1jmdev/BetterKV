# JSON.TOGGLE

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.TOGGLE <key> [path]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
JSON.TOGGLE profile $.active
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
