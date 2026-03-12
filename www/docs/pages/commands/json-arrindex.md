# JSON.ARRINDEX

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.ARRINDEX <key> <path> <scalar> [start [stop]]
```

## Parameters

- `key`: Primary key to read or mutate.
- `path`: JSONPath-like location inside the stored JSON document.
- `scalar`: Command-specific `scalar` argument.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
JSON.ARRINDEX profile $.items "red" 0 10
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
