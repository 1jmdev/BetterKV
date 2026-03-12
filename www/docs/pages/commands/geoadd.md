# GEOADD

Works with geospatial indexes stored in sorted sets.

## Syntax

```text
GEOADD <key> [NX | XX] [CH] <longitude> <latitude> <member> [longitude latitude member ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `longitude`: Command-specific `longitude` argument.
- `latitude`: Command-specific `latitude` argument.
- `member`: Set, sorted-set, or geo member.

## Returns

Returns geo members, distances, coordinates, or a stored-result count depending on the selected options.

## Examples

```bash
GEOADD places 13.4050 52.5200 berlin 2.3522 48.8566 paris
```

## BetterKV Notes

- Group: Geo.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
