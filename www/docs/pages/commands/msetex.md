# MSETEX

Works with string values and bitmap-style string operations.

## Syntax

```text
MSETEX <ttl-seconds> <key> <value> [key value ...]
```

## Parameters

- `ttl`: Command-specific `ttl` argument.
- `key`: Primary key to read or mutate.
- `value`: Value written to BetterKV.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
MSETEX 60 cache:a one cache:b two
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
