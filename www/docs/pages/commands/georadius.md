# GEORADIUS

Works with geospatial indexes stored in sorted sets.

## Syntax

```text
GEORADIUS <key> <longitude> <latitude> <radius> <M | KM | FT | MI> [WITHCOORD] [WITHDIST] [WITHHASH] [COUNT <count> [ANY]] [ASC | DESC] [STORE <key> | STOREDIST <key>]
```

## Parameters

- `key`: Primary key to read or mutate.
- `longitude`: Command-specific `longitude` argument.
- `latitude`: Command-specific `latitude` argument.
- `radius`: Command-specific `radius` argument.
- `count`: Requested number of items.

## Returns

Returns geo members, distances, coordinates, or a stored-result count depending on the selected options.

## Examples

```bash
GEORADIUS places 13.4050 52.5200 200 KM WITHDIST
```

## BetterKV Notes

- Group: Geo.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
