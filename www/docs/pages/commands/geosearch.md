# GEOSEARCH

Works with geospatial indexes stored in sorted sets.

## Syntax

```text
GEOSEARCH <key> [FROMMEMBER <member> | FROMLONLAT <longitude> <latitude>] [BYRADIUS <radius> <M | KM | FT | MI> | BYBOX <width> <height> <M | KM | FT | MI>] [ASC | DESC] [COUNT <count> [ANY]] [WITHCOORD] [WITHDIST] [WITHHASH]
```

## Parameters

- `key`: Primary key to read or mutate.
- `member`: Set, sorted-set, or geo member.
- `longitude`: Command-specific `longitude` argument.
- `latitude`: Command-specific `latitude` argument.
- `radius`: Command-specific `radius` argument.
- `width`: Command-specific `width` argument.
- `height`: Command-specific `height` argument.
- `count`: Requested number of items.

## Returns

Returns geo members, distances, coordinates, or a stored-result count depending on the selected options.

## Examples

```bash
GEOSEARCH places FROMMEMBER berlin BYRADIUS 50 KM WITHCOORD
```

## BetterKV Notes

- Group: Geo.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
