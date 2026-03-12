# MSET

Works with string values and bitmap-style string operations.

## Syntax

```text
MSET <key> <value> [key value ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `value`: Value written to BetterKV.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
MSET app:name BetterKV app:mode production
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
