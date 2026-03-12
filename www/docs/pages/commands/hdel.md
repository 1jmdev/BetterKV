# HDEL

Works with hash fields stored under one key.

## Syntax

```text
HDEL <key> <field> [field ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `field`: Hash field name.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
HDEL user:1 email phone
```

## BetterKV Notes

- Group: Hash.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
