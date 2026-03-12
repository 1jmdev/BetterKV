# RPOPLPUSH

Works with list values.

## Syntax

```text
RPOPLPUSH <source> <destination>
```

## Parameters

- `source`: Source key.
- `destination`: Key that receives the computed result.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
RPOPLPUSH pending processing
```

## BetterKV Notes

- Group: List.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
