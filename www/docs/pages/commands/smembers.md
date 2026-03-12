# SMEMBERS

Works with set values.

## Syntax

```text
SMEMBERS <key>
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an array whose shape depends on the requested items and optional output flags.

## Examples

```bash
SMEMBERS tags
```

## BetterKV Notes

- Group: Set.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
