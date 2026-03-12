# JSON.DEBUG

Operates on JSON documents stored under a BetterKV key.

## Syntax

```text
JSON.DEBUG <HELP | MEMORY> [key [path]]
```

## Parameters

- This command does not require any positional arguments.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
JSON.DEBUG MEMORY profile $
```

## BetterKV Notes

- Group: Keyspace.
- Access: Mixed.
- Status: Implemented in BetterKV with command-specific semantics.
