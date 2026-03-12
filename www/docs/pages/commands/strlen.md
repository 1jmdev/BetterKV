# STRLEN

Works with string values and bitmap-style string operations.

## Syntax

```text
STRLEN <key>
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
STRLEN title
```

## BetterKV Notes

- Group: String.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
