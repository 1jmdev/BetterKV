# PING

Manages client connection state, handshake details, or session-scoped behavior.

## Syntax

```text
PING [message]
```

## Parameters

- This command does not require any positional arguments.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
PING
PING "still alive"
```

## BetterKV Notes

- Group: Connection.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
