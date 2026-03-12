# EXEC

Coordinates transactional execution and optimistic locking.

## Syntax

```text
EXEC
```

## Parameters

- This command does not require any positional arguments.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
MULTI
SET order:1 pending
INCR orders:count
EXEC
```

## BetterKV Notes

- Group: Transaction.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
