# CLIENT

Manages client connection state, handshake details, or session-scoped behavior.

## Syntax

```text
CLIENT <subcommand> [arguments ...]
```

## Parameters

- `subcommand`: Subcommand that selects the command behavior.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
CLIENT LIST
```

## BetterKV Notes

- Group: Connection.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
