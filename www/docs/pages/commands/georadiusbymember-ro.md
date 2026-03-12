# GEORADIUSBYMEMBER_RO

Works with geospatial indexes stored in sorted sets.

## Syntax

```text
GEORADIUSBYMEMBER_RO <key> <member> <radius> <M | KM | FT | MI> [WITHCOORD] [WITHDIST] [WITHHASH] [COUNT <count> [ANY]] [ASC | DESC]
```

## Parameters

- `key`: Primary key to read or mutate.
- `member`: Set, sorted-set, or geo member.
- `radius`: Command-specific `radius` argument.
- `count`: Requested number of items.

## Returns

Returns geo members, distances, coordinates, or a stored-result count depending on the selected options.

## Examples

```bash
GEORADIUSBYMEMBER_RO places berlin 100 KM WITHDIST
```

## BetterKV Notes

- Group: Geo.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
