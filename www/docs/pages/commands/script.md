# SCRIPT

Runs Lua code or manages the script cache.

## Syntax

```text
SCRIPT <subcommand> [arguments ...]
```

## Parameters

- `subcommand`: Subcommand that selects the command behavior.

## Returns

Returns the Lua script result or a script-management reply for the chosen subcommand.

## Examples

```bash
SCRIPT LOAD "return 1"
```

## BetterKV Notes

- Group: Scripting.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
