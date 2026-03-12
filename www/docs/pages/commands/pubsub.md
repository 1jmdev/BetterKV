# PUBSUB

Uses the pub/sub surface for channel-oriented messaging.

## Syntax

```text
PUBSUB <subcommand> [arguments ...]
```

## Parameters

- `subcommand`: Subcommand that selects the command behavior.

## Returns

Returns the standard Redis pub/sub acknowledgement frame or an implementation-specific unsupported error for commands that are not active yet.

## Examples

```bash
PUBSUB CHANNELS metrics:*
```

## BetterKV Notes

- Group: Pub/Sub.
- Access: Declared.
- Status: This command is present in the registry, but the current server implementation still routes it through an unsupported handler. Document it as part of the public surface, but expect a not-supported response until the runtime implementation lands.
