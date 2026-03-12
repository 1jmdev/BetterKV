# RPUSHX

Works with list values.

## Syntax

```text
RPUSHX <key> <element> [element ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `element`: List element value.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
RPUSHX jobs retry-job
```

## BetterKV Notes

- Group: List.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
