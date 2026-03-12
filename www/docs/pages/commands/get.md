# GET

Works with string values and bitmap-style string operations.

## Syntax

```text
GET <key>
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
SET site:name "BetterKV"
GET site:name
```

## BetterKV Notes

- Group: String.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
