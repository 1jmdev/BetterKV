# HELLO

Manages client connection state, handshake details, or session-scoped behavior.

## Syntax

```text
HELLO [protover] [AUTH <username> <password>] [SETNAME <client-name>]
```

## Parameters

- `username`: Command-specific `username` argument.
- `password`: Command-specific `password` argument.
- `client`: Command-specific `client` argument.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
HELLO 3 AUTH default secret SETNAME docs-client
```

## BetterKV Notes

- Group: Connection.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
