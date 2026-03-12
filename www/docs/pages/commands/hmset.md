# HMSET

Works with hash fields stored under one key.

## Syntax

```text
HMSET <key> <field> <value> [field value ...]
```

## Parameters

- `key`: Primary key to read or mutate.
- `field`: Hash field name.
- `value`: Value written to BetterKV.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
HMSET user:2 name Bob role admin
```

## BetterKV Notes

- Group: Hash.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
