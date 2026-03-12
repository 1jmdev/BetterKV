# GEODIST

Works with geospatial indexes stored in sorted sets.

## Syntax

```text
GEODIST <key> <member1> <member2> [M | KM | FT | MI]
```

## Parameters

- `key`: Primary key to read or mutate.
- `member1`: First geo member to compare.
- `member2`: Second geo member to compare.

## Returns

Returns geo members, distances, coordinates, or a stored-result count depending on the selected options.

## Examples

```bash
GEODIST places berlin paris KM
```

## BetterKV Notes

- Group: Geo.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
