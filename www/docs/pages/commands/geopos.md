# GEOPOS

Works with geospatial indexes stored in sorted sets.

## Syntax

```text
GEOPOS <key> <member> [member ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `member`: Set, sorted-set, or geo member.

## Returns

Returns an array whose shape depends on the requested items and optional output flags.

## Examples

```bash
GEOPOS places berlin paris
```

## BetterKV Notes

- Group: Geo.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
