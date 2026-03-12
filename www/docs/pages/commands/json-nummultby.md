# JSON.NUMMULTBY

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.NUMMULTBY <key> <path> <number>
```

## Parameters

- `key`: Primary key to read or mutate.
- `path`: JSONPath-like location inside the stored JSON document.
- `number`: Command-specific `number` argument.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
JSON.NUMMULTBY profile $.factor 1.5
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
