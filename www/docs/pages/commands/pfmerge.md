# PFMERGE

Works with string values and bitmap-style string operations.

## Syntax

```text
PFMERGE <destkey> <sourcekey> [sourcekey ...]
```

## Parameters

- `destkey`: Key that receives the computed result.
- `sourcekey`: Source key.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
PFMERGE visitors:week visitors:mon visitors:tue
```

## BetterKV Notes

- Group: String.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
