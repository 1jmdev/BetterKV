# LCS

Works with string values and bitmap-style string operations.

## Syntax

```text
LCS <key1> <key2> [LEN] [IDX] [MINMATCHLEN <len>] [WITHMATCHLEN]
```

## Parameters

- `key1`: Command-specific `key1` argument.
- `key2`: Command-specific `key2` argument.
- `len`: Command-specific `len` argument.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
LCS ...
```

## BetterKV Notes

- Group: String.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
