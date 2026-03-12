# LREM

Works with list values.

## Syntax

```text
LREM <key> <count> <element>
```

## Parameters

- `key`: Primary key to read or mutate.
- `count`: Requested number of items.
- `element`: List element value.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
LREM queue 2 job-42
```

## BetterKV Notes

- Group: List.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
