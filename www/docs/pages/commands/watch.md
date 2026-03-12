# WATCH

Coordinates transactional execution and optimistic locking.

## Syntax

```text
WATCH <key> [key ...]
```

## Parameters

- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
WATCH wallet:1 wallet:2
```

## BetterKV Notes

- Group: Transaction.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
