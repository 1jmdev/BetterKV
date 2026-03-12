# SELECT

Manages client connection state, handshake details, or session-scoped behavior.

## Syntax

```text
SELECT <db>
```

## Parameters

- `db`: Logical database index.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
SELECT 1
```

## BetterKV Notes

- Group: Connection.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
