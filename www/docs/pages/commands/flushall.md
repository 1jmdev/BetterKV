# FLUSHALL

Inspects or controls server-level behavior.

## Syntax

```text
FLUSHALL [ASYNC | SYNC]
```

## Parameters

- This command does not require any positional arguments.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
FLUSHALL SYNC
```

## BetterKV Notes

- Group: Server.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
