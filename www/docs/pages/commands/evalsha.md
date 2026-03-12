# EVALSHA

Runs Lua code or manages the script cache.

## Syntax

```text
EVALSHA <sha1> <numkeys> [key ...] [arg ...]
```

## Parameters

- `sha1`: SHA1 digest of a previously loaded script.
- `numkeys`: One or more keys processed by the command.

## Returns

Returns the Lua script result or a script-management reply for the chosen subcommand.

## Examples

```bash
EVALSHA abcdef0123456789abcdef0123456789abcdef01 1 profile:1
```

## BetterKV Notes

- Group: Scripting.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
