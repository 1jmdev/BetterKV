# GEOSEARCHSTORE

Works with geospatial indexes stored in sorted sets.

## Syntax

```text
GEOSEARCHSTORE <destination> <source> [FROMMEMBER <member> | FROMLONLAT <longitude> <latitude>] [BYRADIUS <radius> <M | KM | FT | MI> | BYBOX <width> <height> <M | KM | FT | MI>] [ASC | DESC] [COUNT <count> [ANY]] [STOREDIST]
```

## Parameters

- `destination`: Key that receives the computed result.
- `source`: Source key.
- `member`: Set, sorted-set, or geo member.
- `longitude`: Command-specific `longitude` argument.
- `latitude`: Command-specific `latitude` argument.
- `radius`: Command-specific `radius` argument.
- `width`: Command-specific `width` argument.
- `height`: Command-specific `height` argument.

## Returns

Returns geo members, distances, coordinates, or a stored-result count depending on the selected options.

## Examples

```bash
GEOSEARCHSTORE nearby places FROMMEMBER berlin BYRADIUS 25 KM
```

## BetterKV Notes

- Group: Geo.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
