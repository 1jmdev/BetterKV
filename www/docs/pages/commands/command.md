# COMMAND

Inspects or controls server-level behavior.

## Syntax

```text
COMMAND [subcommand [arguments ...]]
```

## Parameters

- This command does not require any positional arguments.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
COMMAND DOCS SET
```

## BetterKV Notes

- Group: Server.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
