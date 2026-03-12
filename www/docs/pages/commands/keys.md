# KEYS

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
KEYS <pattern>
```

## Parameters

- `pattern`: Glob-style match pattern.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
KEYS user:*
```

## BetterKV Notes

- Group: Keyspace.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
