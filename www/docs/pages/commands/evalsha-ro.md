# EVALSHA_RO

Runs Lua code or manages the script cache.

## Syntax

```text
EVALSHA_RO <sha1> <numkeys> [key ...] [arg ...]
```

## Parameters

- `sha1`: SHA1 digest of a previously loaded script.
- `numkeys`: One or more keys processed by the command.

## Returns

Returns the Lua script result or a script-management reply for the chosen subcommand.

## Examples

```bash
EVALSHA_RO ...
```

## BetterKV Notes

- Group: Scripting.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
