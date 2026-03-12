# HSETNX

Works with hash fields stored under one key.

## Syntax

```text
HSETNX <key> <field> <value>
```

## Parameters

- `key`: Primary key to read or mutate.
- `field`: Hash field name.
- `value`: Value written to BetterKV.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
HSETNX user:1 created_at 2026-03-12
```

## BetterKV Notes

- Group: Hash.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
