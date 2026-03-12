# SINTERSTORE

Works with set values.

## Syntax

```text
SINTERSTORE <destination> <key> [key ...]
```

## Parameters

- `destination`: Key that receives the computed result.
- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
SINTERSTORE shared tags:a tags:b
```

## BetterKV Notes

- Group: Set.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
