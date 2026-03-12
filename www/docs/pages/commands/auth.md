# AUTH

Manages client connection state, handshake details, or session-scoped behavior.

## Syntax

```text
AUTH <password>
AUTH <username> <password>
```

## Parameters

- `password`: Command-specific `password` argument.
- `username`: Command-specific `username` argument.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
AUTH default secret-password
```

## BetterKV Notes

- Group: Connection.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
