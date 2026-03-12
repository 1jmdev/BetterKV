# MSETNX

Works with string values and bitmap-style string operations.

## Syntax

```text
MSETNX <key> <value> [key value ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `value`: Value written to BetterKV.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
MSETNX lock:a worker-1 lock:b worker-1
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
