# CONFIG

Inspects or controls server-level behavior.

## Syntax

```text
CONFIG <subcommand> [arguments ...]
```

## Parameters

- `subcommand`: Subcommand that selects the command behavior.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
CONFIG GET *
```

## BetterKV Notes

- Group: Server.
- Access: Declared.
- Status: This command is present in the registry, but the current server implementation still routes it through an unsupported handler. Document it as part of the public surface, but expect a not-supported response until the runtime implementation lands.
