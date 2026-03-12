# ECHO

Manages client connection state, handshake details, or session-scoped behavior.

## Syntax

```text
ECHO <message>
```

## Parameters

- `message`: Message payload to echo or publish.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
ECHO "hello from client"
```

## BetterKV Notes

- Group: Connection.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
