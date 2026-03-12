# LPOS

Works with list values.

## Syntax

```text
LPOS <key> <element> [RANK <rank>] [COUNT <num-matches>] [MAXLEN <len>]
```

## Parameters

- `key`: Primary key to read or mutate.
- `element`: List element value.
- `rank`: Command-specific `rank` argument.
- `num`: Command-specific `num` argument.
- `len`: Command-specific `len` argument.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
LPOS jobs urgent-job COUNT 3
```

## BetterKV Notes

- Group: List.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
