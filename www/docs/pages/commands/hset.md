# HSET

Works with hash fields stored under one key.

## Syntax

```text
HSET <key> <field> <value> [field value ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `field`: Hash field name.
- `value`: Value written to BetterKV.

## Returns

Returns an integer count, length, or mutation result.

## Examples

```bash
HSET user:1 name Alice email alice@example.com
```

## BetterKV Notes

- Group: Hash.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
