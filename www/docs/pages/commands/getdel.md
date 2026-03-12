# GETDEL

Works with string values and bitmap-style string operations.

## Syntax

```text
GETDEL <key>
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
GETDEL one-time-token
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
