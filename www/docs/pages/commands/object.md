# OBJECT

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
OBJECT <subcommand> <key> [arguments ...]
```

## Parameters

- `subcommand`: Subcommand that selects the command behavior.
- `key`: Primary key to read or mutate.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
OBJECT ENCODING user:1
```

## BetterKV Notes

- Group: Keyspace.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
