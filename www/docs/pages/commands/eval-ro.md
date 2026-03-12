# EVAL_RO

Runs Lua code or manages the script cache.

## Syntax

```text
EVAL_RO <script> <numkeys> [key ...] [arg ...]
```

## Parameters

- `script`: Lua script source code.
- `numkeys`: One or more keys processed by the command.

## Returns

Returns the Lua script result or a script-management reply for the chosen subcommand.

## Examples

```bash
EVAL_RO "return redis.call('GET', KEYS[1])" 1 profile:1
```

## BetterKV Notes

- Group: Scripting.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
