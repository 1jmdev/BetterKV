# JSON.MSET

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.MSET <key> <path> <json> [key path json ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `path`: JSONPath-like location inside the stored JSON document.
- `json`: JSON document literal or scalar encoded as JSON.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
JSON.MSET user:1 $.name "Ada" user:2 $.name "Bob"
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
