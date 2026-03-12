# LMOVE

Works with list values.

## Syntax

```text
LMOVE <source> <destination> <LEFT | RIGHT> <LEFT | RIGHT>
```

## Parameters

- `source`: Source key.
- `destination`: Key that receives the computed result.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
LMOVE pending processing RIGHT LEFT
```

## BetterKV Notes

- Group: List.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
