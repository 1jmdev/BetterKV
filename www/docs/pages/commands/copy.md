# COPY

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
COPY <source> <destination> [DB <db>] [REPLACE]
```

## Parameters

- `source`: Source key.
- `destination`: Key that receives the computed result.
- `db`: Logical database index.

## Returns

Returns `OK`, an integer success flag, or a command-specific mutation result.

## Examples

```bash
COPY source:key copied:key REPLACE
```

## BetterKV Notes

- Group: Keyspace.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
