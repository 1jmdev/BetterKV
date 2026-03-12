# BRPOPLPUSH

Works with list values.

## Syntax

```text
BRPOPLPUSH <source> <destination> <timeout>
```

## Parameters

- `source`: Source key.
- `destination`: Key that receives the computed result.
- `timeout`: Blocking timeout in seconds or milliseconds depending on the command.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
BRPOPLPUSH ...
```

## BetterKV Notes

- Group: List.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
