# LLEN

Works with list values.

## Syntax

```text
LLEN <key>
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
LLEN jobs
```

## BetterKV Notes

- Group: List.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
