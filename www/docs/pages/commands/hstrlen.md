# HSTRLEN

Works with hash fields stored under one key.

## Syntax

```text
HSTRLEN <key> <field>
```

## Parameters

- `key`: Primary key to read or mutate.
- `field`: Hash field name.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
HSTRLEN user:1 name
```

## BetterKV Notes

- Group: Hash.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
