# GEORADIUS_RO

Works with geospatial indexes stored in sorted sets.

## Syntax

```text
GEORADIUS_RO <key> <longitude> <latitude> <radius> <M | KM | FT | MI> [WITHCOORD] [WITHDIST] [WITHHASH] [COUNT <count> [ANY]] [ASC | DESC]
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
GEORADIUS_RO places 13.4050 52.5200 50 KM WITHDIST
```

## BetterKV Notes

- Group: Geo.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
